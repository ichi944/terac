import { useState } from 'react'
import logo from './logo.svg'
import './App.css'

function App() {

  const handleClick = () => {
    console.log('ok')
    fetch('/graphql', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
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
        `
      })
    })
    .then(res => res.json())
    .then(json => console.log('result', json))
  }
  return (
    <div className="App">
      <header className="App-header">
        <button onClick={handleClick}>Fetch!</button>
      </header>
    </div>
  )
}

export default App
