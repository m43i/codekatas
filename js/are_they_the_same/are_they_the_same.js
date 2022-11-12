function comp(array1, array2){
    if (!array1 || !array2) {
      return false;
    }
    return array1.length === array2.length && array1.map((x) => x * x).sort().every((v, i) => array2.sort()[i] === v);
}