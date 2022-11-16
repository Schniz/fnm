export default function getStderr(script: string): string {
  return `${script} 2>&1`
}
