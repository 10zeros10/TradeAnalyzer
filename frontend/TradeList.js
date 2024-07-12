import React, { useState, useEffect } from 'react';
import axios from 'axios';

const fetchTradeResults = async () => {
  try {
    const url = `${process.env.REACT_APP_BACKEND_URL}/trade-results`;
    const response = await axios.get(url);
    return response.data;
  } catch (error) {
    console.error('Failed to fetch trade results:', error);
    return [];
  }
};

const TradeResultsList = () => {
  const [tradeResults, setTradeResults] = useState([]);

  useEffect(() => {
    const initFetch = async () => {
      const results = await fetchTradeResults();
      setTradeResults(results);
    };

    initFetch();
  }, []);

  return (
    <div>
      <h2>Analyzed Trade Results</h2>
      <ul>
        {tradeResults.map(result => (
          <li key={result.id}>
            <p>Volume: {result.volume}</p>
            <p>Average Price: {result.averagePrice}</p>
            <p>Timestamp: {new Date(result.timestamp).toLocaleString()}</p>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default TradeResultsList;