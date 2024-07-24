const memoize = (fn) => {
  let cache = {};
  return (...args) => {
    const n = args[0];  // assuming single argument for simplicity
    if (n in cache) {
        console.log('Fetching from cache');
        return cache[n];
    }
    else {
        console.log('Calculating result');
        let result = fn(n);
        cache[n] = result;
        return result;
    }
  }
}

// Example of a function you want to memoize
const expensiveCalculation = n => {
  // Simulate heavy work
  for(let i = 0; i < 10000; i++) {}
  return n * n; 
}

// Creating a memoized version of the expensiveCalculation function
const memoizedExpensiveCalculation = memoize(expensiveCalculation);

// Usage Example
console.log(memoizedExpensiveCalculation(9));  // Calculating result
console.log(memoizedExpensiveCalculation(9));  // Fetching from cache

// This is a simple component that could benefit from memoization if re-rendered often with the same props
const MyComponent = React.memo(function MyComponent(props) {
  // Component implementation
  return <div>{props.children}</div>;
});

import React, { useMemo } from 'react';

const MyComponent = ({ someValue }) => {
  const expensiveValue = useMemo(() => {
    return expensiveCalculation(someValue);
  }, [someValue]);  // Dependency array, useMemo will only recompute if someValue changes

  return <div>{expensiveValue}</div>;
};