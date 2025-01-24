// revisit swap logic

type Data = {
  name: string;
  age: number;
};

const arr: Data[] = [
  { name: "jordan", age: 24 },
  { name: "sarah", age: 31 },
  { name: "dylan", age: 19 },
];

function bubbleSort<T>(
  arr: T[],
  comparator: (a: T, b: T) => boolean = (a, b) => a > b
): T[] {
  let sortedArr = [...arr];
  let swapped: boolean;
  for (let i = 0; i < sortedArr.length - 1; i++) {
    for (let j = 0; j < sortedArr.length - 1 - i; j++) {
      if (comparator(sortedArr[j], sortedArr[j + 1])) {
        [sortedArr[j], sortedArr[j + 1]] = [sortedArr[j + 1], sortedArr[j]];
        swapped = true;
      }
    }
    if (!swapped) break;
  }
  return sortedArr;
}

console.log(bubbleSort<Data>(arr, (a, b) => a.age < b.age));
