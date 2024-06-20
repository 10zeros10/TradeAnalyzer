import React, { Component } from 'react';
import TradeForm from './TradeForm';
import TradeList from './TradeList';

class TradeAnalysisApp extends Component {
  constructor(props) {
    super(props);
    this.state = {
      // State for holding the trade data
      trades: [],
    };
  }

  // Function to add a trade to the trades array in the state
  addTrade = (tradeData) => {
    this.setState(prevState => ({
      trades: [...prevState.trades, tradeData],
    }));
  };

  render() {
    return (
      <div className="trade-analysis-app">
        <h1>Trade Analysis Application</h1>
        {/* TradeForm for uploading trade data, passing addTrade as a prop */}
        <TradeForm addTrade={this.addTrade} />
        {/* TradeList for displaying analyzed results, passing trades as a prop */}
        <TradeList trades={this.state.trades} />
      </div>
    );
  }
}

export default TradeAnalysisApp;