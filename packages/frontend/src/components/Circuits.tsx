import { useContext, useState, useEffect } from "react";
import { WASMContext } from "../context/wasm";

export const HelloWorld = () => {
  const ctx = useContext(WASMContext);
  const wasm = ctx.wasm!;
  if (!wasm) return <></>;

  console.log(wasm.simulate(10, 100.0, 10.0, 300, 300).filter((x) => x > 0));
  return <h1> hello from rust</h1>;
};
