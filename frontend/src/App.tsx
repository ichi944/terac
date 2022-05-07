import React, { useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import { ChakraProvider, StatHelpTextProps } from "@chakra-ui/react";
import { Home } from "./components/Home";
import { AppDrawer } from "./components/AppDrawer";
import {
  RecoilRoot,
} from "recoil";

function App() {

  const handleClick = () => {
    console.log("ok");
    fetch("/graphql", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        query: `
          query {
            users {
              id
              email
              results {
                courseId
              }
            }
          }
        `,
      }),
    })
      .then((res) => res.json())
      .then((json) => console.log("result", json));
  };
  return (
    <React.StrictMode>
      <RecoilRoot>
        <ChakraProvider>
          <div className="App">
            <header className="App-header">
              <Home />
              <button onClick={handleClick}>Fetch!</button>
            </header>
          </div>
        </ChakraProvider>
      </RecoilRoot>
    </React.StrictMode>
  );
}

export default App;
