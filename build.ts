import ts from 'typescript';
import path from 'path';

await Bun.$`rm -rf dist`;

await Bun.$`echo Building...`;

await Bun.build({
  entrypoints: ['index.ts'],
  outdir: 'dist',
  minify: true,
})

const configPath = path.resolve('tsconfig.publish.json');
const configFile = ts.readConfigFile(configPath, ts.sys.readFile);

if (configFile.error) {
  console.error('Error reading tsconfig.publish.json');
  process.exit(1);
}

const parsedCommandLine = ts.parseJsonConfigFileContent(
  configFile.config,
  ts.sys,
  path.dirname(configPath)
);

if (parsedCommandLine.errors.length > 0) {
  console.error('Error parsing tsconfig.publish.json');
  process.exit(1);
}

const program = ts.createProgram({
  rootNames: parsedCommandLine.fileNames,
  options: parsedCommandLine.options,
});

await program.emit();

await Bun.$`echo Build completed!`;