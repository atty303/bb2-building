import {EditorView, basicSetup} from "codemirror";
import {EditorState} from "@codemirror/state";
import {markdown} from "@codemirror/lang-markdown";

export class CodeMirror {
    constructor(parent, onChange) {
        const updateListener = EditorView.updateListener.of((vu) => {
            if (vu.docChanged) {
                const doc = vu.state.doc;
                const value = doc.toString();
                onChange(value);
            }
        });

        const state = EditorState.create({
            extensions: [basicSetup, markdown(), updateListener],
        });

        this.view = new EditorView({
            state,
            parent,
        });
    }

    get value() {
        return this.view.state.doc.toString();
    }

    set value(value) {
        this.view.dispatch({changes: {from: 0, to: this.view.state.doc.length, insert: value}});
    }
}