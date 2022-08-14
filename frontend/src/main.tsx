import React from "react";
import ReactDOM from "react-dom/client";
import { RecoilRoot } from "recoil";
import {
  Link,
  MakeGenerics,
  Outlet,
  ReactLocation,
  Router,
  Navigate,
  useMatch,
} from "@tanstack/react-location";
import { ChakraProvider } from "@chakra-ui/react";
import App from "./App";
import "./index.css";

const Foo = () => <div>This is Foo Element.</div>;
const AppContent = () => <div>This is AppContent Element.</div>;
const routes = [
  {
    path: "app",
    children: [
      {
        path: "/",
        element: <AppContent />,
      },
      {
        path: "foo",
        element: <Foo />,
      },
    ],
  },
];
const location = new ReactLocation();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <RecoilRoot>
      <ChakraProvider>
        <Router location={location} routes={routes}>
          <App />
        </Router>
      </ChakraProvider>
    </RecoilRoot>
  </React.StrictMode>
);
