/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use crate::config::Config;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum Intervention {
    Vaccinate(Vaccinate),
    Lockdown(Lockdown),
    BuildNewHospital(BuildNewHospital)
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Vaccinate {
    pub at_hour: i32,
    pub percent: f64,
}

impl Vaccinate {
    pub fn new(at_hour: i32, percent: f64) -> Vaccinate {
        Vaccinate { at_hour, percent }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Lockdown {
    pub at_number_of_infections: i32,
    pub emergency_workers_population: f64
}

impl Lockdown {
    pub fn new(at_number_of_infections: i32, emergency_workers_population: f64) -> Lockdown {
        Lockdown { at_number_of_infections, emergency_workers_population }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct BuildNewHospital {
    pub spread_rate_threshold: i32
}

impl Intervention {
    pub fn get_hospital_intervention(config: &Config) -> Option<BuildNewHospital> {
        return config.get_interventions().iter().filter_map(|i| {
            match i {
                Intervention::BuildNewHospital(x) => Some(x),
                _ => None
            }
        }).next().copied();
    }
}
