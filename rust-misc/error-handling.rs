fn main() {
    println!("Hallo main method");

    //unsichere Ausführung der Funktion bei direktem Abwickeln(Unwrap)
    let result: i8 = error_check(false).unwrap(); 
    println!("{}", result);

    //verarbeitet das Ergebnis nicht direkt, sondern übergibt es an describe_result 
    let safe_result: Result<i8, &'static str> = error_check(true); 
    describe_result(safe_result);

    let _result: i8 = error_check(true).expect("dies ist aufgefangen worden");

    let safe_result: Result<i8, &'static str> = error_check(true);

    //Prüfen Sie, ob einen Fehler vorliegt und druck ihn gegenebenfalls aus.
    if safe_result.is_err() {
        println!("Das ist ein anderer Fehler"); 
    }

}

fn error_check(check: bool) -> Result<i8, &'static str> {
    if check == true {
        return Err("Wir haben einen Fehler gemacht")
    } else {
        return Ok(1)
    }
}
//Result ist der Typ, der für die Rückgabe und Weitergabe von Fehlern verwendet wird.
//diese Funktion nimmt ein Ergebnis und behandelt es mit einer Übereinstimmungsanweisung.
fn describe_result(result: Result<i8, &'static str>){
    //switch-case
    match result {
        Ok(x) => println!("Das ist ein Ergebnis von :  {}", x),
        Err(x) => println!("{}", x)
    }
}