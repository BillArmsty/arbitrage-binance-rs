# arbitrage-binance-rs
Be able to gather data about trades for 3 coin pairs from the Binance WebSocket API. Then, it will calculate potential profits if we were to trade the pairs in sequence.
The arbitrage attempts to take advantage of price discrepancies between 3 assets that form a triangle on the same exchange. An example trade would be: BTC -> ETH -> BNB -> BTC.

