import { describe, expect, it } from "vitest";
import { set } from "./lib";

describe("set", () => {
  it("should set number", () => {
    const obj = { name: "/", files: [], directories: [] };
    set(obj, "foo", 100);
    expect(obj).toEqual({
      name: "/",
      files: [{ name: "foo", size: 100 }],
      directories: [],
    });
  });

  it("should set multiple", () => {
    const obj = { name: "/", files: [], directories: [] };
    set(obj, "foo", 100);
    expect(obj).toEqual({
      name: "/",
      files: [{ name: "foo", size: 100 }],
      directories: [],
    });

    set(obj, "bar", 100);
    expect(obj).toEqual({
      name: "/",
      files: [
        { name: "foo", size: 100 },
        { name: "bar", size: 100 },
      ],
      directories: [],
    });
  });

  it("should set nested", () => {
    const obj = { name: "/", files: [], directories: [] };
    set(obj, "foo\\bar", 100);
    expect(obj).toEqual({
      name: "/",
      files: [],
      directories: [
        { name: "foo", files: [{ name: "bar", size: 100 }], directories: [] },
      ],
    });
  });

  it("should set deeply nested", () => {
    const obj = { name: "/", files: [], directories: [] };
    set(obj, "foo\\bar\\baz\\quu\\qux", 100);
    console.log(JSON.stringify(obj));
    expect(obj).toEqual({
      name: "/",
      files: [],
      directories: [
        {
          name: "foo",
          files: [],
          directories: [
            {
              name: "bar",
              files: [],
              directories: [
                {
                  name: "baz",
                  files: [],
                  directories: [
                    {
                      name: "quu",
                      files: [{ name: "qux", size: 100 }],
                      directories: [],
                    },
                  ],
                },
              ],
            },
          ],
        },
      ],
    });
  });
});
