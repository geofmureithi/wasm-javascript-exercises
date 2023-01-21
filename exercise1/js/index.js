function fibonacci(term) {
    // TODO: Add code here
    var sequence = [];

    sequence.push(0);
    sequence.push(1);


    for (var i = 2; i <= term; i++) {
        var next_num = (sequence[i - 1] + sequence[i - 2]);

        sequence.push(next_num);
        // console.log(sequence);

    }
    var sum = 0;
    for (var j=0; j< sequence.length; j++) {
        sum+=sequence[j];

    }
    console.log(sequence);

    return sum;

    // let sum = sequence.join();
    // console.log(sum);
    // return sum;
}

// export default fibonacci;
fibonacci(10);
