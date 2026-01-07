//! Sistema de Agendamento de Gravação
//!
//! Permite definir quando gravar, com que qualidade e que ações tomar.
//! Suporta agendamentos simples (horas do dia) e complexos (feriados, exceções).

use crate::types::CameraId;
use crate::media_profile::MediaProfileId;
use chrono::{DateTime, NaiveTime, Utc, Weekday, Datelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// ID único de um agendamento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScheduleId(pub Uuid);

impl ScheduleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ScheduleId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ScheduleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Modo de gravação
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingMode {
    /// Não gravar
    None,
    /// Gravação contínua
    Continuous,
    /// Gravar apenas quando movimento detectado
    Motion,
    /// Gravar apenas quando evento dispara
    Event,
    /// Gravar por movimento OU evento
    MotionOrEvent,
}

impl Default for RecordingMode {
    fn default() -> Self {
        Self::Continuous
    }
}

/// Tipo de dia para agendamento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayType {
    /// Dia de semana normal
    Weekday(Weekday),
    /// Qualquer dia de semana (seg-sex)
    AnyWeekday,
    /// Fim de semana (sab-dom)
    Weekend,
    /// Feriado customizado
    Holiday,
    /// Exceção específica (data específica)
    Exception,
    /// Todos os dias
    Everyday,
}

/// Período de tempo dentro de um dia
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriod {
    /// Hora de início
    pub start: NaiveTime,
    /// Hora de fim
    pub end: NaiveTime,
}

impl TimePeriod {
    pub fn new(start: NaiveTime, end: NaiveTime) -> Self {
        Self { start, end }
    }

    /// Período que cobre o dia inteiro
    pub fn all_day() -> Self {
        Self {
            start: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        }
    }

    /// Horário comercial padrão (8h-18h)
    pub fn business_hours() -> Self {
        Self {
            start: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        }
    }

    /// Período noturno (18h-6h)
    pub fn night_hours() -> Self {
        Self {
            start: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        }
    }

    /// Verifica se um horário está dentro do período
    pub fn contains(&self, time: NaiveTime) -> bool {
        if self.start <= self.end {
            // Período normal dentro do mesmo dia
            time >= self.start && time <= self.end
        } else {
            // Período que cruza meia-noite
            time >= self.start || time <= self.end
        }
    }
}

/// Configuração de gravação para um período
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingScheduleEntry {
    /// Período de tempo
    pub period: TimePeriod,

    /// Modo de gravação
    pub mode: RecordingMode,

    /// Perfil de mídia a usar (None = usar padrão)
    pub profile_id: Option<MediaProfileId>,

    /// Habilitar pré-gravação (buffer antes do evento)
    pub pre_recording_seconds: u32,

    /// Continuar gravando após movimento/evento parar
    pub post_recording_seconds: u32,

    /// Habilitar análise de IA
    pub ai_enabled: bool,

    /// Prioridade (maior = mais importante)
    pub priority: u8,
}

impl RecordingScheduleEntry {
    pub fn continuous_all_day() -> Self {
        Self {
            period: TimePeriod::all_day(),
            mode: RecordingMode::Continuous,
            profile_id: None,
            pre_recording_seconds: 5,
            post_recording_seconds: 10,
            ai_enabled: false,
            priority: 5,
        }
    }

    pub fn motion_only() -> Self {
        Self {
            period: TimePeriod::all_day(),
            mode: RecordingMode::Motion,
            profile_id: None,
            pre_recording_seconds: 5,
            post_recording_seconds: 10,
            ai_enabled: true,
            priority: 5,
        }
    }
}

/// Agendamento para um tipo de dia
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaySchedule {
    /// Tipo de dia
    pub day_type: DayType,

    /// Entradas de agendamento para este dia
    pub entries: Vec<RecordingScheduleEntry>,
}

impl DaySchedule {
    pub fn new(day_type: DayType) -> Self {
        Self {
            day_type,
            entries: Vec::new(),
        }
    }

    /// Adiciona 24/7 contínuo
    pub fn with_continuous_all_day(mut self) -> Self {
        self.entries.push(RecordingScheduleEntry::continuous_all_day());
        self
    }

    /// Adiciona período personalizado
    pub fn with_entry(mut self, entry: RecordingScheduleEntry) -> Self {
        self.entries.push(entry);
        self
    }
}

/// Feriado customizado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holiday {
    /// Nome do feriado
    pub name: String,

    /// Data (formato MM-DD para recorrente, YYYY-MM-DD para específico)
    pub date: String,

    /// É recorrente anualmente?
    pub recurring: bool,
}

impl Holiday {
    pub fn recurring(name: &str, month: u32, day: u32) -> Self {
        Self {
            name: name.to_string(),
            date: format!("{:02}-{:02}", month, day),
            recurring: true,
        }
    }

    pub fn specific(name: &str, year: i32, month: u32, day: u32) -> Self {
        Self {
            name: name.to_string(),
            date: format!("{}-{:02}-{:02}", year, month, day),
            recurring: false,
        }
    }

    pub fn matches(&self, date: DateTime<Utc>) -> bool {
        let date_str = if self.recurring {
            format!("{:02}-{:02}", date.month(), date.day())
        } else {
            format!("{}-{:02}-{:02}", date.year(), date.month(), date.day())
        };
        date_str == self.date
    }
}

/// Agendamento completo de gravação para uma câmera
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingSchedule {
    /// ID do agendamento
    pub id: ScheduleId,

    /// Nome do agendamento
    pub name: String,

    /// Câmeras que usam este agendamento
    pub camera_ids: Vec<CameraId>,

    /// Agendamentos por dia da semana
    pub daily_schedules: HashMap<Weekday, DaySchedule>,

    /// Agendamento padrão (quando não há específico)
    pub default_schedule: DaySchedule,

    /// Agendamento para feriados
    pub holiday_schedule: Option<DaySchedule>,

    /// Lista de feriados
    pub holidays: Vec<Holiday>,

    /// Exceções (datas específicas com agendamento diferente)
    pub exceptions: HashMap<String, DaySchedule>,

    /// Dias de retenção (ciclo de gravação)
    pub retention_days: u32,

    /// Está ativo
    pub is_active: bool,
}

impl RecordingSchedule {
    pub fn new(name: &str) -> Self {
        Self {
            id: ScheduleId::new(),
            name: name.to_string(),
            camera_ids: Vec::new(),
            daily_schedules: HashMap::new(),
            default_schedule: DaySchedule::new(DayType::Everyday).with_continuous_all_day(),
            holiday_schedule: None,
            holidays: Vec::new(),
            exceptions: HashMap::new(),
            retention_days: 30,
            is_active: true,
        }
    }

    /// Template: 24/7 contínuo
    pub fn template_24_7() -> Self {
        Self::new("24/7 Continuous Recording")
    }

    /// Template: Horário comercial
    pub fn template_business_hours() -> Self {
        let mut schedule = Self::new("Business Hours Only");

        let weekday_schedule = DaySchedule::new(DayType::AnyWeekday)
            .with_entry(RecordingScheduleEntry {
                period: TimePeriod::business_hours(),
                mode: RecordingMode::Continuous,
                profile_id: None,
                pre_recording_seconds: 5,
                post_recording_seconds: 10,
                ai_enabled: false,
                priority: 5,
            })
            .with_entry(RecordingScheduleEntry {
                period: TimePeriod::night_hours(),
                mode: RecordingMode::Motion,
                profile_id: None,
                pre_recording_seconds: 10,
                post_recording_seconds: 30,
                ai_enabled: true,
                priority: 5,
            });

        schedule.default_schedule = weekday_schedule;
        schedule
    }

    /// Template: Apenas movimento
    pub fn template_motion_only() -> Self {
        let mut schedule = Self::new("Motion Detection Only");
        schedule.default_schedule = DaySchedule::new(DayType::Everyday)
            .with_entry(RecordingScheduleEntry::motion_only());
        schedule
    }

    /// Adiciona câmera ao agendamento
    pub fn add_camera(&mut self, camera_id: CameraId) {
        if !self.camera_ids.contains(&camera_id) {
            self.camera_ids.push(camera_id);
        }
    }

    /// Adiciona feriado brasileiro padrão
    pub fn add_brazilian_holidays(&mut self) {
        self.holidays.extend(vec![
            Holiday::recurring("Ano Novo", 1, 1),
            Holiday::recurring("Tiradentes", 4, 21),
            Holiday::recurring("Dia do Trabalho", 5, 1),
            Holiday::recurring("Independência", 9, 7),
            Holiday::recurring("Nossa Senhora Aparecida", 10, 12),
            Holiday::recurring("Finados", 11, 2),
            Holiday::recurring("Proclamação da República", 11, 15),
            Holiday::recurring("Natal", 12, 25),
        ]);
    }

    /// Obtém a configuração efetiva para um momento específico
    pub fn get_effective_config(&self, dt: DateTime<Utc>) -> Option<&RecordingScheduleEntry> {
        if !self.is_active {
            return None;
        }

        let time = dt.time();
        let weekday = dt.weekday();
        let date_str = format!("{}-{:02}-{:02}", dt.year(), dt.month(), dt.day());

        // 1. Verificar exceções (mais específico)
        if let Some(exception_schedule) = self.exceptions.get(&date_str) {
            for entry in &exception_schedule.entries {
                if entry.period.contains(time) {
                    return Some(entry);
                }
            }
        }

        // 2. Verificar feriados
        let is_holiday = self.holidays.iter().any(|h| h.matches(dt));
        if is_holiday {
            if let Some(ref holiday_schedule) = self.holiday_schedule {
                for entry in &holiday_schedule.entries {
                    if entry.period.contains(time) {
                        return Some(entry);
                    }
                }
            }
        }

        // 3. Verificar agendamento do dia da semana
        if let Some(day_schedule) = self.daily_schedules.get(&weekday) {
            for entry in &day_schedule.entries {
                if entry.period.contains(time) {
                    return Some(entry);
                }
            }
        }

        // 4. Usar agendamento padrão
        for entry in &self.default_schedule.entries {
            if entry.period.contains(time) {
                return Some(entry);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_time_period() {
        let period = TimePeriod::business_hours();
        let morning = NaiveTime::from_hms_opt(10, 0, 0).unwrap();
        let night = NaiveTime::from_hms_opt(22, 0, 0).unwrap();

        assert!(period.contains(morning));
        assert!(!period.contains(night));
    }

    #[test]
    fn test_night_period_crosses_midnight() {
        let period = TimePeriod::night_hours();
        let late_night = NaiveTime::from_hms_opt(23, 0, 0).unwrap();
        let early_morning = NaiveTime::from_hms_opt(3, 0, 0).unwrap();
        let afternoon = NaiveTime::from_hms_opt(14, 0, 0).unwrap();

        assert!(period.contains(late_night));
        assert!(period.contains(early_morning));
        assert!(!period.contains(afternoon));
    }

    #[test]
    fn test_schedule_templates() {
        let schedule = RecordingSchedule::template_24_7();
        let dt = Utc::now();
        assert!(schedule.get_effective_config(dt).is_some());
    }

    #[test]
    fn test_holiday_matching() {
        let holiday = Holiday::recurring("Natal", 12, 25);
        let christmas = Utc.with_ymd_and_hms(2024, 12, 25, 12, 0, 0).unwrap();
        let regular_day = Utc.with_ymd_and_hms(2024, 12, 24, 12, 0, 0).unwrap();

        assert!(holiday.matches(christmas));
        assert!(!holiday.matches(regular_day));
    }
}
