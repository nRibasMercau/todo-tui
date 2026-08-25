use chrono::{Datelike, Days, NaiveDate};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::calendar::{CalendarEventStore, Monthly},
};
use time::{Date, Month, error::ComponentRange};

fn to_time_date(date: NaiveDate) -> Result<time::Date, ComponentRange> {
    time::Date::from_calendar_date(
        date.year(),
        time::Month::try_from(date.month() as u8).unwrap(),
        date.day() as u8,
    )
}

pub fn next_month(date: NaiveDate) -> NaiveDate {
    if date.month() == 12 {
        date.with_month(1)
            .unwrap()
            .with_year(date.year() + 1)
            .unwrap()
    } else {
        date.with_month(date.month() + 1).unwrap()
    }
}

pub fn prev_month(date: NaiveDate) -> NaiveDate {
    if date.month() == 1 {
        date.with_month(12)
            .unwrap()
            .with_year(date.year() - 1)
            .unwrap()
    } else {
        date.with_month(date.month() - 1).unwrap()
    }
}

pub fn move_down(date: NaiveDate) -> NaiveDate {
    date.checked_add_days(Days::new(7)).unwrap()
}

pub fn move_up(date: NaiveDate) -> NaiveDate {
    date.checked_sub_days(Days::new(7)).unwrap()
}

pub fn move_right(date: NaiveDate) -> NaiveDate {
    date.checked_add_days(Days::new(1)).unwrap()
}

pub fn move_left(date: NaiveDate) -> NaiveDate {
    date.checked_sub_days(Days::new(1)).unwrap()
}

/// Makes a list of dates for the current year.
fn events(selected_date: NaiveDate) -> Result<CalendarEventStore, ComponentRange> {
    const SELECTED: Style = Style::new()
        .fg(Color::White)
        .bg(Color::Red)
        .add_modifier(Modifier::BOLD);
    const HOLIDAY: Style = Style::new()
        .fg(Color::Red)
        .add_modifier(Modifier::UNDERLINED);
    const SEASON: Style = Style::new()
        .fg(Color::Green)
        .bg(Color::Black)
        .add_modifier(Modifier::UNDERLINED);

    let mut list = CalendarEventStore::today(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Blue),
    );
    let y = selected_date.year();
    let date = to_time_date(selected_date).unwrap();

    // new year's
    list.add(Date::from_calendar_date(y, Month::January, 1)?, HOLIDAY);
    // next new_year's for December "show surrounding"
    list.add(Date::from_calendar_date(y + 1, Month::January, 1)?, HOLIDAY);
    // groundhog day
    list.add(Date::from_calendar_date(y, Month::February, 2)?, HOLIDAY);
    // april fool's
    list.add(Date::from_calendar_date(y, Month::April, 1)?, HOLIDAY);
    // earth day
    list.add(Date::from_calendar_date(y, Month::April, 22)?, HOLIDAY);
    // star wars day
    list.add(Date::from_calendar_date(y, Month::May, 4)?, HOLIDAY);
    // festivus
    list.add(Date::from_calendar_date(y, Month::December, 23)?, HOLIDAY);
    // new year's eve
    list.add(Date::from_calendar_date(y, Month::December, 31)?, HOLIDAY);

    // seasons
    // spring equinox
    list.add(Date::from_calendar_date(y, Month::March, 22)?, SEASON);
    // summer solstice
    list.add(Date::from_calendar_date(y, Month::June, 21)?, SEASON);
    // fall equinox
    list.add(Date::from_calendar_date(y, Month::September, 22)?, SEASON);
    // winter solstice
    list.add(Date::from_calendar_date(y, Month::December, 21)?, SEASON);

    // selected date
    list.add(date, SELECTED);

    Ok(list)
}

/// Render the UI with a calendar.
pub fn render(frame: &mut Frame, calendar_area: Rect, selected_date: NaiveDate) {
    let date = to_time_date(selected_date).unwrap();

    let events = match events(selected_date) {
        Ok(events) => events,
        Err(error) => {
            eprintln!("Error creating calendar events: {error}");
            return;
        }
    };

    render_month(frame, calendar_area, date, &events);
}

fn render_month(frame: &mut Frame, calendar_area: Rect, date: Date, events: &CalendarEventStore) {
    let calendar = Monthly::new(date, events)
        .default_style(Style::new().bold().bg(Color::Rgb(50, 50, 50)))
        .show_month_header(Style::new().bold().green())
        .show_surrounding(Style::new().dim())
        .show_weekdays_header(Style::new().bold().light_yellow());

    frame.render_widget(calendar, calendar_area);
}
