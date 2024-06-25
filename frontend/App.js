import React, { useState } from 'react';
import TradeForm from './TradeForm';
import TradeList from './TradeList';

const TradeAnalysisApp = () => {
  const [trades, setTrades] = useState([]);
  const [error, setError] = useState('');

  const addTrade = (tradeData) => {
    try {
      if (!tradeData || typeof tradeData !== 'object') {
        throw new Error('Invalid trade data');
      }
      setTrades((prevTrades) => [...prevTrades, tradeData]);
      if (error) setError(''); 
    } catch (err) {
      setError(err.message);
    }
  };

  return (
    <div className="trade-analysis-app">
      <h1>Trade Analysis Application</h1>
      {error && <div className="error">{error}</div>}
      <TradeForm addTrade={addTrade} />
      <TradeList trades={trades} />
    </div>
  );
};

export default TradeAnalysisApp;