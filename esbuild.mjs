import * as esbuild from 'esbuild';

await esbuild.build({
    entryPoints: ['src/package.mjs'],
    bundle: true,
    outfile: 'src/bundle.js',
    minify: true,
    format: 'esm',
});