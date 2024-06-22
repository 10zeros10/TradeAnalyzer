import React, { useState } from 'react';
import TradeForm from './TradeForm';
import TradeList from './TradeList';

const TradeAnalysisApp = () => {
  const [trades, setTrades] = useState([]);

  const addTrade = (tradeData) => {
    setTrades((prevTrades) => [...prevTrades, tradeData]);
  };

  return (
    <div className="trade-analysis-app">
      <h1>Trade Analysis Application</h1>
      <TradeForm addTrade={addTrade} />
      <TradeList trades={trades} />
    </div>
  );
};

export default TradeAnalysisApp;