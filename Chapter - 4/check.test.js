const assert = require("assert");

describe("calculateDaysBetweenDates", () => {
  it("should return 0 when begin and end dates are the same", () => {
    const begin = "2022-01-01";
    const end = "2022-01-01";
    const expected = 0;
    const result = calculateDaysBetweenDates(begin, end);
    assert.strictEqual(result, expected);
  });

  it("should return the correct number of days when begin date is before end date", () => {
    const begin = "2022-01-01";
    const end = "2022-01-05";
    const expected = 4;
    const result = calculateDaysBetweenDates(begin, end);
    assert.strictEqual(result, expected);
  });

  it("should return the correct number of days when begin date is after end date", () => {
    const begin = "2022-01-05";
    const end = "2022-01-01";
    const expected = 4;
    const result = calculateDaysBetweenDates(begin, end);
    assert.strictEqual(result, expected);
  });
});
