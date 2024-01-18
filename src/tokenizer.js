var segmenter = undefined;

export class IntlTokenizer {
    constructor(locale) {
        this.segmenter = new Intl.Segmenter(locale, {granularity: "word"});
    }

    tokenize(text) {
        return [...this.segmenter.segment(text)[Symbol.iterator]()].map(({segment}) => segment);
    }
}
