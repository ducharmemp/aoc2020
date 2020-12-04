import { readFile } from "fs/promises";
import { fromPairs, every, toPairs, difference } from "lodash";

const PASSPORT_FIELDS = {
  byr: (value: string) => value.length === 4 && 1920 <= parseInt(value, 10) && parseInt(value, 10) <= 2002,
  iyr: (value: string) => value.length === 4 && 2010 <= parseInt(value, 10) && parseInt(value, 10) <= 2020,
  eyr: (value: string) => value.length === 4 && 2020 <= parseInt(value, 10) && parseInt(value, 10) <= 2030,
  hgt: (value: string) => {
    const match = value.match(/^(\d+)(cm|in)$/);
    if (match === null) {
      return false;
    }
    const parsed = parseInt(match[1], 10);
    switch (match[2]) {
      case "cm":
        return 150 <= parsed && parsed <= 193;
      case "in":
        return 59 <= parsed && parsed <= 76;
    }
  },
  hcl: (value: string) => /^#[\w\d]{6}$/.test(value),
  ecl: (value: string) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].includes(value),
  pid: (value: string) => /^\d{9}$/.test(value),
};

function buildPassportLineGroups(lines: string[]): string[][] {
  const groups: string[][] = [[]];

  lines.forEach((line) => {
    if (line.trim() == "") {
      groups.push([]);
    } else {
      groups[groups.length - 1].push(...line.split(" "));
    }
  });

  return groups;
}

function findPairs(passport: string[]): Record<string, string> {
  return fromPairs(passport.map((entry) => entry.split(":")).filter(([key, _]) => key != "cid"));
}

function validPassport(checkValues: boolean, passport: string[]): boolean {
  const entries = findPairs(passport);
  const checks = toPairs(entries).map(([key, value]) => PASSPORT_FIELDS[key as keyof typeof PASSPORT_FIELDS](value));
  return difference(Object.keys(PASSPORT_FIELDS), Object.keys(entries)).length === 0 && (every(checks) || !checkValues);
}

async function main(): Promise<void> {
  const input = (await readFile("./input.txt")).toString("utf-8");
  const lines = input.split(/\r?\n/);

  const groups = buildPassportLineGroups(lines);
  console.log(groups.filter(validPassport.bind(null, false)).length);
  console.log(groups.filter(validPassport.bind(null, true)).length);
}

main();
