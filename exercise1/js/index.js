function fibonacci(term) {
    // TODO: Add code here
    var sequence = [];

    sequence.push(0);
    sequence.push(1);


    for (var i = 2; i <= term; i++) {
        var next_num = (sequence[i - 1] + sequence[i - 2]);

        sequence.push(next_num);

    }
    var sum = 0;
    for (var j=0; j< sequence.length; j++) {
        sum+=sequence[j];

    }
    // console.log(next_num);

    return next_num;

   
}

export default fibonacci;
// fibonacci(9);
