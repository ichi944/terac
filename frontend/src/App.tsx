import React, { useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import { ChakraProvider } from "@chakra-ui/react";
import { Home } from "./components/Home";
import { AppDrawer } from "./components/AppDrawer";

function App() {
  const [drawerIsOpen, toggleDrawerIsOpen] = useState<boolean>(false);
  const handleToggleDrawer = () => {
    toggleDrawerIsOpen(!drawerIsOpen);
  };
  const onClose = () => {
    toggleDrawerIsOpen(false);
  };
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
      <ChakraProvider>
        <div className="App">
          <AppDrawer isOpen={drawerIsOpen} onClose={onClose} />
          <header className="App-header">
            <Home />
            <button onClick={handleClick}>Fetch!</button>
            <button onClick={handleToggleDrawer}>Toggle!</button>
          </header>
        </div>
      </ChakraProvider>
    </React.StrictMode>
  );
}

export default App;
