import { describe, expect, it } from "vitest";
import { cn, formatDateRange, initials } from "./utils";

describe("cn", () => {
  it("joins truthy class names", () => {
    expect(cn("a", "b")).toBe("a b");
  });

  it("filters falsy values", () => {
    expect(cn("a", false, null, undefined, "b")).toBe("a b");
    expect(cn(false, null, undefined)).toBe("");
  });
});

describe("formatDateRange", () => {
  it("returns empty when both missing", () => {
    expect(formatDateRange()).toBe("");
    expect(formatDateRange(undefined, undefined)).toBe("");
  });

  it("uses Present when only start is set", () => {
    expect(formatDateRange("2020")).toBe("2020 — Present");
  });

  it("returns end alone when start missing", () => {
    expect(formatDateRange(undefined, "2024")).toBe("2024");
  });

  it("formats full range", () => {
    expect(formatDateRange("2020", "2024")).toBe("2020 — 2024");
  });
});

describe("initials", () => {
  it("takes up to two name parts", () => {
    expect(initials("Ada Lovelace")).toBe("AL");
    expect(initials("Madonna")).toBe("M");
    expect(initials("Jean Luc Picard")).toBe("JL");
  });

  it("handles extra whitespace", () => {
    expect(initials("  Ada   Lovelace  ")).toBe("AL");
  });

  it("returns empty for blank name", () => {
    expect(initials("")).toBe("");
    expect(initials("   ")).toBe("");
  });
});
