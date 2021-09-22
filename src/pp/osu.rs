use cpython::{ToPyObject, PyDict, Python};
use std::fs::File;
use rosu_pp::{Beatmap, OsuPP, BeatmapExt};

pub(crate) struct StdResults {
    total_stars: f32,
    partial_stars: f32,
    pp: f32,
    max_pp: f32
}

impl ToPyObject for StdResults {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "total_stars", self.total_stars).unwrap();
        dict.set_item(py, "partial_stars", self.partial_stars).unwrap();
        dict.set_item(py, "pp", self.pp).unwrap();
        dict.set_item(py, "max_pp", self.max_pp).unwrap();

        dict
    }
}

pub(crate) fn calculate_std_pp(map: String, mods: u32, combo: Option<usize>, acc: Option<f32>,
                               potential_acc: Option<f32>, n300: Option<usize>, n100: Option<usize>,
                               n50: Option<usize>, nmiss: Option<usize>,
                               passed_objects: Option<usize>) -> StdResults {
    let file = match File::open(map) {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };

    let map = match Beatmap::parse(file) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };

    let result = OsuPP::new(&map)
        .mods(mods);

    let result = match combo {
        Some(x) => result.combo(x),
        None => result
    };

    let result = match acc {
        Some(x) => result.accuracy(x),
        None => result
    };

    let result = match n300 {
        Some(x) => result.n300(x),
        None => result
    };

    let result = match n100 {
        Some(x) => result.n100(x),
        None => result
    };

    let result = match n50 {
        Some(x) => result.n50(x),
        None => result
    };

    let result = match passed_objects {
        Some(x) => result.passed_objects(x),
        None => result
    };

    let result = match nmiss {
        Some(x) => result.misses(x),
        None => result.misses(0)
    };

    let new_result = OsuPP::new(&map)
        .mods(mods)
        .misses(0);

    let new_result = match potential_acc {
        Some(x) => new_result.accuracy(x),
        None => new_result
    };

    let stats = StdResults {
        total_stars: map.stars(mods, None).stars(),
        partial_stars: map.stars(mods, passed_objects).stars(),
        pp: result.calculate().pp(),
        max_pp: new_result.calculate().pp()
    };

    return stats;
}