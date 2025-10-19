let content = "";

let sum1 = content
	.split("\n")
	.map(line => line.match(/\d/g))
	.map(matches => [parseInt(matches[0], 10), parseInt(matches[matches.length - 1], 10)])
	.map(([d, n]) => d * 10 + n)
	.reduce((total, curr) => total + curr, 0);
