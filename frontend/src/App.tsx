import React from "react";
import "./App.css";
import { Initializer } from "./components/auth/Initializer";
import { Home } from "./components/Home";
import { useRecoilValue } from "recoil";
import { InitializeStateAtom } from "./atoms/InitializeStateAtom";

import {
  Link,
  MakeGenerics,
  Outlet,
  ReactLocation,
  Router,
  Navigate,
  useMatch,
} from "@tanstack/react-location";

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
  if (!useRecoilValue(InitializeStateAtom).initialized) {
    return <Initializer />;
  }
  const authorized = true;
  if (!authorized) {
    return <Navigate to="/auth"></Navigate>;
  }
  return (
    <React.StrictMode>
      <div className="App">
        <header className="App-header">
          <Link to="app">go to app</Link>
          <Link to="app/foo">go to Foo</Link>
          <Home />
          <button onClick={handleClick}>Fetch!!!!</button>
        </header>
      </div>
      <Outlet />
    </React.StrictMode>
  );
}

export default App;
