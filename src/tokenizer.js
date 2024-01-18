var segmenter = undefined;

export function tokenize(text) {
    if (!segmenter) {
        segmenter = new Intl.Segmenter("ja", {granularity: "word"});
    }
    return [...segmenter.segment(text)[Symbol.iterator]()].map(({segment}) => segment);
}