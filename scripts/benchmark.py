import subprocess
import os
import re
import datetime
import json

def get_platform_info():
    try:
        uname = os.uname()
        os_info = f"{uname.sysname} {uname.release}"
    except:
        os_info = "Unknown OS"

    cpu_info = "Unknown CPU"
    try:
        with open("/proc/cpuinfo", "r") as f:
            for line in f:
                if "model name" in line:
                    cpu_info = line.split(":")[1].strip()
                    break
    except:
        pass

    ram_info = "Unknown RAM"
    try:
        with open("/proc/meminfo", "r") as f:
            for line in f:
                if "MemTotal" in line:
                    kb = int(line.split(":")[1].strip().split(" ")[0])
                    ram_info = f"{kb // 1024 // 1024} GB"
                    break
    except:
        pass

    return f"{os_info}, {cpu_info}, {ram_info}"

def parse_cargo_bench():
    print("Running cargo bench... This may take several minutes.")

    # Run tests on paperpilot-pdf and paperpilot-nlp
    env = os.environ.copy()

    process = subprocess.Popen(
        ["cargo", "bench", "-p", "paperpilot-pdf", "-p", "paperpilot-nlp"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        env=env
    )

    output = []
    for line in iter(process.stdout.readline, ''):
        print(line, end='')
        output.append(line)

    process.wait()
    return "".join(output)

def parse_metrics(output):
    metrics = {
        "memory": {}, # key: Op_Size, value: peak bytes
        "latency": {}, # key: Op_Size, value: {p50, p95, p99}
        "throughput": {}, # key: Op_Size, value: throughput elements/s
        "cold_warm": {}, # key: Op, value: {cold, warm}
        "leak": {}, # key: Op_Iter, value: bytes
        "concurrent": {} # key: Mode, value: time
    }

    # We will read criterion's JSON output for latency
    # Wait, criterion output is usually text, we can parse it, or rely on target/criterion/.../new/estimates.json

    # Let's parse custom print output first
    for line in output.split('\n'):
        if line.startswith("SYSTEM_BENCHMARK_MEMORY|"):
            parts = line.split("|")
            op_size = parts[1]
            mem = int(parts[2])
            metrics["memory"][op_size] = mem
        elif line.startswith("SYSTEM_BENCHMARK_COLDWARM|"):
            parts = line.split("|")
            op = parts[1]
            cold = int(parts[2])
            warm = int(parts[3])
            metrics["cold_warm"][op] = {"cold": cold, "warm": warm}
        elif line.startswith("SYSTEM_BENCHMARK_LEAK|"):
            parts = line.split("|")
            op = parts[1]
            it = int(parts[2])
            mem = int(parts[3])
            if op not in metrics["leak"]:
                metrics["leak"][op] = {}
            metrics["leak"][op][it] = mem
        elif line.startswith("SYSTEM_BENCHMARK_CONCURRENT|"):
            parts = line.split("|")
            mode = parts[1]
            time = int(parts[2])
            metrics["concurrent"][mode] = time

    # Parse criterion estimates from disk
    # Paths are like target/criterion/{group}/{name}/new/estimates.json
    # where {group} is like Merge, Split
    # and {name} is like 1, 10, 50, etc. or merge_20_docs_100_pages

    base_dir = "target/criterion"
    if os.path.exists(base_dir):
        for group in os.listdir(base_dir):
            if group == "report": continue
            group_dir = os.path.join(base_dir, group)
            if not os.path.isdir(group_dir): continue

            for name in os.listdir(group_dir):
                if name == "report": continue
                name_dir = os.path.join(group_dir, name)
                est_file = os.path.join(name_dir, "new", "estimates.json")
                if os.path.exists(est_file):
                    with open(est_file, 'r') as f:
                        data = json.load(f)
                        p50 = data.get("Median", {}).get("point_estimate", 0)

                        # read sample.json to calculate P95, P99
                        sample_file = os.path.join(name_dir, "new", "sample.json")
                        p95 = p50
                        p99 = p50
                        iters = []
                        if os.path.exists(sample_file):
                            with open(sample_file, 'r') as sf:
                                sdata = json.load(sf)
                                iters = sdata.get("iters", [])
                                times = sdata.get("times", [])

                                if len(iters) > 0 and len(times) > 0:
                                    avg_times = [t / i for t, i in zip(times, iters)]
                                    avg_times.sort()
                                    p95 = avg_times[int(len(avg_times) * 0.95) - 1] if len(avg_times) > 0 else 0
                                    p99 = avg_times[int(len(avg_times) * 0.99) - 1] if len(avg_times) > 0 else 0

                        op_size = f"{group}_{name}"
                        # Some mappings
                        if name == "merge_20_docs_100_pages":
                            op_size = f"{group}_100"
                        elif name == "ocr_large_100page":
                            op_size = f"{group}_100"
                        elif name == "render_medium_50page":
                            op_size = f"{group}_50"
                        elif name == "10000_queries":
                            op_size = f"{group}_10000"

                        metrics["latency"][op_size] = {
                            "p50": p50 / 1_000_000, # ms
                            "p95": p95 / 1_000_000,
                            "p99": p99 / 1_000_000,
                            "raw_p50_ns": p50
                        }

                        # throughput
                        # for simple, elements/s
                        if len(iters) > 0 and len(times) > 0:
                            total_time = sum(times) / 1e9
                            elements = 0
                            if name.isdigit():
                                elements = int(name) * sum(iters)
                            elif name == "merge_20_docs_100_pages":
                                elements = 100 * sum(iters)
                            elif name == "ocr_large_100page":
                                elements = 100 * sum(iters)
                            elif name == "render_medium_50page":
                                elements = 50 * sum(iters)
                            elif name == "10000_queries":
                                elements = 10000 * sum(iters)

                            if total_time > 0:
                                metrics["throughput"][op_size] = elements / total_time

    return metrics

def format_time(ms):
    if ms < 1:
        return f"{ms*1000:.2f} µs"
    elif ms > 1000:
        return f"{ms/1000:.2f} s"
    return f"{ms:.2f} ms"

def generate_report(metrics, platform_info):
    ts = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    report = [
        "# PaperPilot — System Benchmark Report",
        f"Generated: {ts}",
        f"Platform: {platform_info}",
        "",
        "## Core Operations Scaling (Latency by Page Count)",
        "| Operation | 1 page | 10 pages | 50 pages | 100 pages | 500 pages |",
        "|---|---|---|---|---|---|"
    ]

    ops = ["Merge", "Split", "ExtractText", "Compress", "Watermark", "Encrypt", "OCR", "Render"]
    sizes = ["1", "10", "50", "100", "500"]

    for op in ops:
        row = [op]
        for size in sizes:
            key = f"{op}_{size}"
            if key in metrics["latency"]:
                row.append(format_time(metrics["latency"][key]["p50"]))
            else:
                row.append("-")
        report.append("| " + " | ".join(row) + " |")

    report.extend([
        "",
        "## Core Operations — P50/P95/P99 Latency (Max Size)",
        "| Operation | P50 | P95 | P99 | Throughput (pages/s) |",
        "|---|---|---|---|---|"
    ])

    for op in ops:
        # get max size
        max_size = None
        for size in reversed(sizes):
            key = f"{op}_{size}"
            if key in metrics["latency"]:
                max_size = size
                break

        if max_size:
            key = f"{op}_{max_size}"
            lat = metrics["latency"][key]
            tp = metrics["throughput"].get(key, 0)
            report.append(f"| {op} | {format_time(lat['p50'])} | {format_time(lat['p95'])} | {format_time(lat['p99'])} | {tp:.2f} |")

    report.extend([
        "",
        "## Cold Start vs Warm Start",
        "| Operation | Cold Start | Warm (avg) | Delta |",
        "|---|---|---|---|"
    ])

    for op, cw in metrics["cold_warm"].items():
        cold_ms = cw["cold"] / 1000.0
        warm_ms = cw["warm"] / 1000.0
        delta = cold_ms - warm_ms
        report.append(f"| {op} | {format_time(cold_ms)} | {format_time(warm_ms)} | {format_time(delta)} |")

    report.extend([
        "",
        "## Memory Leak Detection (1000 Iterations)",
        "| Operation | Iteration | RSS (MB) |",
        "|---|---|---|"
    ])

    for op in metrics["leak"]:
        for it in sorted(metrics["leak"][op].keys()):
            mb = metrics["leak"][op][it] / (1024*1024)
            report.append(f"| {op} | {it} | {mb:.2f} MB |")

    report.extend([
        "",
        "## Concurrent Operations",
        "| Mode | Total Time |",
        "|---|---|"
    ])

    if "Sequential" in metrics["concurrent"]:
        report.append(f"| Sequential | {format_time(metrics['concurrent']['Sequential'] / 1000.0)} |")
    if "Concurrent" in metrics["concurrent"]:
        report.append(f"| Concurrent | {format_time(metrics['concurrent']['Concurrent'] / 1000.0)} |")

    report.extend([
        "",
        "## NLP Resolver Throughput (10,000 queries)",
        "| Metric | Value |",
        "|---|---|"
    ])

    nlp_key = "NlpResolver_10000"
    if nlp_key in metrics["latency"]:
        lat = metrics["latency"][nlp_key]
        tp = metrics["throughput"].get(nlp_key, 0)
        report.append(f"| Average Latency (per query) | {format_time(lat['p50'] / 10000.0)} |")
        report.append(f"| P99 Latency (batch) | {format_time(lat['p99'])} |")
        report.append(f"| Throughput | {tp:.2f} queries/s |")

    return "\n".join(report)

def main():
    platform_info = get_platform_info()
    output = parse_cargo_bench()
    metrics = parse_metrics(output)
    report_md = generate_report(metrics, platform_info)

    with open("SYSTEM_BENCHMARK_REPORT.md", "w") as f:
        f.write(report_md)

    print("\nBenchmark report generated: SYSTEM_BENCHMARK_REPORT.md")

if __name__ == "__main__":
    main()
