# AI Matrix Test Results

_Last updated: 29/07/2025, 05:37:32_

## Matrix Table

**Test Configuration:**
Total games played: 144
Duration: 1.70 seconds
Games per second: 84.8

| AI Type   | Random | Heuristic | MM-Depth1 | MM-Depth2 | MM-Depth3 | MM-Depth4 | MM-Depth5 | MM-Depth6 | ML-Simple |
| --------- | ------ | --------- | --------- | --------- | --------- | --------- | --------- | --------- | --------- |
| Random    | -      | 0.0       | 0.0       | 25.0      | 25.0      | 75.0      | 50.0      | 25.0      | 0.0       |
| Heuristic | 100.0  | -         | 25.0      | 0.0       | 75.0      | 100.0     | 0.0       | 0.0       | 100.0     |
| MM-Depth1 | 100.0  | 75.0      | -         | 25.0      | 50.0      | 100.0     | 50.0      | 50.0      | 0.0       |
| MM-Depth2 | 75.0   | 100.0     | 75.0      | -         | 0.0       | 50.0      | 75.0      | 50.0      | 25.0      |
| MM-Depth3 | 75.0   | 25.0      | 50.0      | 100.0     | -         | 75.0      | 50.0      | 0.0       | 50.0      |
| MM-Depth4 | 25.0   | 0.0       | 0.0       | 50.0      | 25.0      | -         | 25.0      | 0.0       | 0.0       |
| MM-Depth5 | 50.0   | 100.0     | 50.0      | 25.0      | 50.0      | 75.0      | -         | 75.0      | 50.0      |
| MM-Depth6 | 75.0   | 100.0     | 50.0      | 50.0      | 100.0     | 100.0     | 25.0      | -         | 100.0     |
| ML-Simple | 100.0  | 0.0       | 100.0     | 75.0      | 50.0      | 100.0     | 50.0      | 0.0       | -         |

## Performance Summary

1. MM-Depth6: 75.0% average win rate
2. MM-Depth5: 59.4% average win rate
3. ML-Simple: 59.4% average win rate
4. MM-Depth1: 56.2% average win rate
5. MM-Depth2: 56.2% average win rate
6. MM-Depth3: 53.1% average win rate
7. Heuristic: 50.0% average win rate
8. Random: 25.0% average win rate
9. MM-Depth4: 15.6% average win rate

## Speed Analysis

| AI        | ms/move | Speed     |
| --------- | ------- | --------- |
| Random    | 0.0     | Very Fast |
| Heuristic | 0.0     | Very Fast |
| MM-Depth1 | 0.0     | Very Fast |
| ML-Simple | 0.0     | Very Fast |
| MM-Depth2 | 0.0     | Very Fast |
| MM-Depth3 | 2.9     | Fast      |
| MM-Depth4 | 20.3    | Moderate  |
| MM-Depth5 | 58.2    | Slow      |
| MM-Depth6 | 173.5   | Slow      |

## Recommendations

- MM-Depth6 shows excellent performance (75.0% avg win rate) and is ready for production
- MM-Depth2 is very fast (0.0ms/move) and suitable for real-time play
- Use MM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
