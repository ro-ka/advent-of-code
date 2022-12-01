function getValues(input: string) {
	return input
		.split('\n\n')
		.map(group => group
			.split('\n')
			.reduce((sum, value) => sum + Number(value), 0)
		);
}


export function part1(input: string) {
	return Math.max(...getValues(input));
}

export function part2(input: string) {
	return getValues(input)
		.sort((a, b) =>  b - a)
		.slice(0, 3)
		.reduce((sum, value) => sum + value, 0);
}