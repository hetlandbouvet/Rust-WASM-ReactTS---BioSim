import React, { useEffect, useRef, useState } from 'react';
import logo from './logo.svg';
import './index.css';
import axios from 'axios';

export interface AnimalCount {
  year: number,
  num_herbs: number,
  num_carns: number
}



function App() {

  const [animalCount, setAnimalCount] = useState(0);
  const result = '/result.json';
  const apiCall = () => {
    axios.get(result)
      .then(resp => {      
        resp.data.res.map((y: AnimalCount) => console.log(y.year))
      })
  }
  useEffect(() => {
    setInterval(() => apiCall(), 10000)
  })
  
  // apiCall();
  return (
    <div className="App">
      <div className="flex space-x-2 justify-center">
        <button 
        // onClick={() => init().then(() => greet("test"))}
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
          {animalCount}
        </button>      
      </div>  
        {/* <AnimalLineChart count={animalCount}/> */}
    </div>
  );
}

export default App;
