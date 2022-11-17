(async () => {
  for await (const chunk of process.stdin) {
    const letters = chunk.toString("utf8").split("");
    for (const letter of letters) {
      process.stdout.write(letter);
      await sleep(Math.random() * 100 + 20);
    }
  }
})();

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
