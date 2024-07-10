import React, { useState, useEffect } from 'react';
import axios from 'axios';

const TradeResultsList = () => {
  const [tradeResults, setTradeResults] = useState([]);

  useEffect(() => {
    const fetchTradeResults = async () => {
      try {
        const response = await axios.get(`${process.env.REACT_APP_BACKEND_URL}/trade-results`);
        setTradeResults(response.data);
      } catch (error) {
        console.error('Failed to fetch trade results:', error);
      }
    };

    fetchTradeResults();
  }, []);

  return (
    <div>
      <h2>Analyzed Trade Results</h2>
      <ul>
        {trade Read moreesults.map(result => (
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