import * as esbuild from 'esbuild';

await esbuild.build({
    entryPoints: ['packages/app/src/package.mjs'],
    bundle: true,
    outfile: 'packages/app/src/bundle.js',
    minify: true,
    format: 'esm',
});