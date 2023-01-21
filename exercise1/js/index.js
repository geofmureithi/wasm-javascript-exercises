function fibonacci(term) {
    // TODO: Add code here
    var sequence = [];
    let next_num = 0;

    sequence.push(0);
    sequence.push(1);


    for (var i = 2; i <= term; i++) {
        next_num = (sequence[i - 1] + sequence[i - 2]);

        sequence.push(next_num);

    }
   

    console.log(next_num);
    return next_num;

   
}

// export default fibonacci;
fibonacci(9);
