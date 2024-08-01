use std::rc::Rc;
#[derive(Debug)]
struct City{
    name :  Rc<String>,
    population : i32,
    history:Rc<String>
}

#[derive(Debug)]
struct CityHistories{
    city_names:Vec<Rc<String>>,
    histories:Vec<Rc<String>>
}

pub fn test_city(){
    let city_name =  Rc::new(String::from("Tirunelveli"));
    let city_history =  Rc::new(String::from("திருநெல்வேலி அல்லது நெல்லை (ஒலிப்புⓘ, Tirunelveli), என்பது இந்தியாவின், தமிழ்நாடு மாநிலத்தில் அமைந்துள்ள திருநெல்வேலி மாவட்டத்தில் இருக்கும் ஒரு மாநகராட்சி ஆகும். திக்கெல்லாம் புகழுறும் திருநெல்வேலி' எனச் சம்பந்தரும், தண் பொருநைப் புனல்நாடு' எனச் சேக்கிழாரும், பொன்திணிந்த புனல் பெருகும் பொருநைத் திருநதி' என்று கம்பரும் பாடிய பூமி, திருநெல்வேலி ஆகும். இவ்வூர் அல்வாவிற்கு பெயர் பெற்றது."));
    let tirunelveli = City{
        name: Rc::clone(&city_name),
        population:1242367,
        history: Rc::clone(&city_history)
    };

    let varalaru = CityHistories{
        city_names: vec![Rc::clone(&city_name)],
        histories: vec![Rc::clone(&city_history)]
    };

    println!("{varalaru:?}  {tirunelveli:?}");
    println!("{}",Rc::strong_count(&city_name));
}