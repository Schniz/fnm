#!/usr/bin/env node

const { execSync } = require("child_process");

const groupByMap = (xs, fn, mapFn) => {
  const grouped = {};

  for (const x of xs) {
    const key = fn(x);
    grouped[key] = grouped[key] || [];
    grouped[key].push(mapFn(x));
  }

  return grouped;
};

const licenses = execSync('grep -r "license:" esy.lock')
  .toString()
  .split("\n")
  .map(x => x.trim())
  .filter(Boolean)
  .map(line => {
    const [path, , license] = line.split(":");
    const dependency = path
      .split("/")
      .slice(1)
      .join("/")
      .match(/^(.+)\/[^/]+$/)[1];
    return { dependency, license: JSON.parse(license.trim()) };
  });

const byLicense = groupByMap(
  licenses,
  ({ license }) => license,
  ({ dependency }) => dependency
);

for (const [license, packages] of Object.entries(byLicense)) {
  console.log(`${license}:`);
  for (const package of packages) {
    console.log(`  - ${package}`);
  }
}
