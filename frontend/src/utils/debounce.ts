export function debounce<T extends (...args: any[]) => any>(
  func: T,
  delay: number
): (...args: Parameters<T>) => Promise<Awaited<ReturnType<T>>> {
  let timeout: number | null = null;
  return function (this: unknown, ...args: Parameters<T>) {
    return new Promise((resolve) => {
      if (timeout !== null) clearTimeout(timeout);
      timeout = setTimeout(() => resolve(func.apply(this, args)), delay);
    });
  };
}
