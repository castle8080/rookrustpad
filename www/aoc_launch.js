import { LitElement, html } from 'lit';

export class AOCLauncher extends LitElement {
    
    static properties = {
        problems: []
    }

    constructor() {
        super();
        this.problems = [];
    }

    render() {
        return html`
            <h2>Advent of Code Launcher</h2>
            <div>${this.problems}</div>
        `;
    }

}

customElements.define('aoc-launcher', AOCLauncher);
