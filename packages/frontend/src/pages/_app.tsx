import "../styles/globals.css";
import type { AppProps } from "next/app";
import { WASMContextProvider } from "../context/wasm";

const App = ({ Component, pageProps }: AppProps) => {
  return (
    <WASMContextProvider>
      <Component {...pageProps} />
    </WASMContextProvider>
  );
};

export default App;
