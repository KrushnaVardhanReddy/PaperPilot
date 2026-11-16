import { mount } from 'svelte'
import './app.css'
import App from './App.svelte'

const target = document.getElementById('app')!
// Clear the SEO pre-rendered crawler fallback before mounting the interactive app
target.replaceChildren()

const app = mount(App, {
  target,
})

export default app
