use genpdf::elements::{LinearLayout, PageBreak, TableLayout};
use genpdf::style::Style;
use genpdf::Scale;
use genpdf::{elements, fonts, style, Alignment, Element, Margins, Mm};

struct RowData {
    text: String,
    style: Style,
    alignment: Alignment,
}

impl RowData {
    fn new(text: &str, style: Style, alignment: Alignment) -> Self {
        Self {
            text: text.to_string(),
            style,
            alignment,
        }
    }
}

fn add_empty_row(table: &mut TableLayout) {
    (*table)
        .row()
        .element(elements::Break::new(0))
        .push()
        .unwrap();
}

fn build_table<E: Element + 'static>(data: Vec<E>, padding: Margins) -> TableLayout {
    let mut table = elements::TableLayout::new(vec![1]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, false, false));

    for d in data {
        table.row().element(d.padded(padding)).push().unwrap();
    }

    table
}

fn row_table(data: Vec<RowData>) -> elements::TableLayout {
    let mut table = elements::TableLayout::new(data.iter().map(|_| 1).collect());
    table.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let vertical_padding = Mm::from(2);
    let zero = Mm::from(0);
    let padding = Margins::trbl(zero, zero, vertical_padding, zero);

    let mut row = table.row();
    for d in data {
        row = row.element(
            elements::Paragraph::new(d.text)
                .aligned(d.alignment)
                .styled(d.style)
                .padded(padding),
        )
    }
    row.push().unwrap();

    table
}

fn linear_layout<E: Element + 'static>(title: &str, subtitle: &str, element: E) -> LinearLayout {
    let mut layout = elements::LinearLayout::vertical();
    let bipa_color = style::Color::Rgb(0, 206, 120);
    let vertical_padding = Mm::from(2);
    let zero = Mm::from(0);
    let padding = Margins::trbl(zero, zero, vertical_padding, zero);
    let black = style::Color::Rgb(28, 28, 30);

    layout.push(
        elements::Paragraph::new(title)
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(12))
            .styled(bipa_color)
            .padded(padding),
    );

    layout.push(
        elements::Paragraph::new(subtitle)
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(12))
            .padded(padding)
            .styled(black),
    );

    layout.push(element);

    layout
}

fn btc_linear_layout<E: Element + 'static>(
    title: &str,
    subtitle: &str,
    element: E,
) -> LinearLayout {
    let mut layout = elements::LinearLayout::vertical();
    let btc_color = style::Color::Rgb(68, 87, 212);
    let vertical_padding = Mm::from(2);
    let zero = Mm::from(0);
    let padding = Margins::trbl(zero, zero, vertical_padding, zero);
    let black = style::Color::Rgb(28, 28, 30);

    layout.push(
        elements::Paragraph::new(title)
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(14))
            .styled(black)
            .padded(padding),
    );

    layout.push(
        elements::Paragraph::new(subtitle)
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(12))
            .padded(padding)
            .styled(btc_color),
    );

    layout.push(element);

    layout
}

fn gold_linear_layout<E: Element + 'static>(subtitle: &str, element: E) -> LinearLayout {
    let mut layout = elements::LinearLayout::vertical();
    let color = style::Color::Rgb(255, 204, 0);
    let vertical_padding = Mm::from(2);
    let zero = Mm::from(0);
    let padding = Margins::trbl(zero, zero, vertical_padding, zero);
    let black = style::Color::Rgb(28, 28, 30);

    layout.push(
        elements::Paragraph::new(subtitle)
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(12))
            .styled(color)
            .padded(padding),
    );

    layout.push(element);

    layout
}

fn pdf() {
    let default_font = fonts::from_files("assets/Roboto", "Roboto", None)
        .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    decorator.set_header(|page| {
        let mut layout = elements::LinearLayout::vertical();
        if page > 1 {
            layout.push(
                elements::Paragraph::new(format!("Página {}", page)).aligned(Alignment::Center),
            );
            layout.push(elements::Break::new(1));
        }
        layout.styled(style::Style::new().with_font_size(10))
    });
    doc.set_page_decorator(decorator);

    let mut table = elements::TableLayout::new(vec![1, 1]);
    let mut image = elements::Image::from_path("assets/bipa-logo.jpg").unwrap();
    image.set_scale(Scale::new(0.3, 0.3));
    image.set_alignment(Alignment::Left);

    let gold_color = style::Color::Rgb(255, 204, 0);
    let bipa_color = style::Color::Rgb(0, 206, 120);
    let btc_color = style::Color::Rgb(68, 87, 212);
    let gray4 = style::Color::Rgb(174, 174, 178);
    let black = style::Color::Rgb(28, 28, 30);

    table
        .row()
        .element(image)
        .element(
            elements::Paragraph::new("Informe de Rendimentos Financeiro")
                .aligned(Alignment::Right)
                .styled(style::Style::new().bold().with_font_size(14))
                .styled(bipa_color),
        )
        .push()
        .expect("Invalid table row");

    doc.push(table);

    doc.push(elements::Break::new(1.0));

    doc.push(
        elements::Paragraph::new("Imposto de Renda - Pessoa Física")
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(16)),
    );

    let layout = linear_layout(
        "Idenficação da Fonte Pagadora",
        "Acesso Soluções de Pagamento S.A.",
        elements::Paragraph::default()
            .styled_string("CNPJ:", gray4)
            .string(" 13.140.088/0001-99")
            .styled(style::Style::new().with_font_size(12))
            .styled(black),
    );

    let layout1 = linear_layout(
        "Pessoa Física Beneficiária dos Rendimentos",
        "Felipe Rosa",
        elements::Paragraph::default()
            .styled_string("CPF:", gray4)
            .string(" 000.000.000-00")
            .styled_string("    Agência:", gray4)
            .string(" 0001")
            .styled_string("    Conta:", gray4)
            .string(" 0020332")
            .styled(style::Style::new().with_font_size(12))
            .styled(black),
    );

    let rows = vec![layout, layout1];
    let vertical_padding = Mm::from(5);
    let zero = Mm::from(0);
    let padding = Margins::trbl(vertical_padding, zero, vertical_padding, zero);

    let mut table = build_table(rows, padding);
    add_empty_row(&mut table);

    let gray3 = style::Color::Rgb(199, 199, 204);
    let table = table.styled(gray3);
    doc.push(elements::Break::new(1.0));
    doc.push(table);
    doc.push(elements::Break::new(1));

    let data1 = vec![
        RowData::new(
            "Rendimentos Sujeitos a Tributação Exclusiva",
            Style::new().with_color(black).with_font_size(12).bold(),
            Alignment::Left,
        ),
        RowData::new(
            "Valores em R$",
            Style::new().with_color(gray3).with_font_size(10),
            Alignment::Right,
        ),
    ];
    let row1 = row_table(data1);

    let leading_row_style = Style::new().with_color(gray4).with_font_size(10);
    let trailing_row_style = Style::new().with_color(black).with_font_size(10).bold();
    let data2 = vec![
        RowData::new("Saldo em 31/12/2020", leading_row_style, Alignment::Left),
        RowData::new("R$0", trailing_row_style, Alignment::Right),
    ];
    let row2 = row_table(data2);

    let data3 = vec![
        RowData::new("Saldo em 31/12/2021", leading_row_style, Alignment::Left),
        RowData::new("R$0", trailing_row_style, Alignment::Right),
    ];
    let row3 = row_table(data3);

    let leading_row_style = Style::new().with_color(bipa_color).with_font_size(8);
    let trailing_row_style = Style::new()
        .with_color(bipa_color)
        .with_font_size(10)
        .bold();
    let data4 = vec![
        RowData::new("Rendimentos Líquidos", leading_row_style, Alignment::Left),
        RowData::new("R$0", trailing_row_style, Alignment::Right),
    ];
    let row4 = row_table(data4);

    let vertical_padding = Mm::from(3);
    let padding = Margins::trbl(vertical_padding, zero, vertical_padding, zero);
    let rows = vec![row1, row2, row3, row4];
    let table = build_table(rows, padding);
    let table = table.styled(gray3);

    doc.push(table);

    doc.push(elements::PageBreak::new());

    let layout = linear_layout(
        "Idenficação da Fonte Compradora e Custodiante de Criptoativos",
        "Bipa Intermediação de Ativos Digitais LTDA",
        elements::Paragraph::default()
            .styled_string("CNPJ:", gray4)
            .string(" 37.008.710/0001-78")
            .styled(style::Style::new().with_font_size(12))
            .styled(black),
    );

    let rows = vec![layout];
    let vertical_padding = Mm::from(5);
    let zero = Mm::from(0);
    let padding = Margins::trbl(vertical_padding, zero, vertical_padding, zero);

    let mut table = build_table(rows, padding);
    add_empty_row(&mut table);

    let table = table.styled(gray3);
    doc.push(table);

    let layout = btc_linear_layout(
        "Bens e Direitos",
        "81 - Criptoativo Bitcoin - BTC",
        elements::Paragraph::default()
            .styled_string("Data:", gray4)
            .string(" 31/12/2021")
            .styled(style::Style::new().with_font_size(12))
            .styled(black),
    );

    let rows = vec![layout];
    let vertical_padding = Mm::from(5);
    let zero = Mm::from(0);
    let padding = Margins::trbl(vertical_padding, zero, vertical_padding, zero);

    let table = build_table(rows, padding);
    doc.push(table);

    let table_header_style = Style::new().with_color(gray3).with_font_size(10);
    let table_value = Style::new().with_color(black).with_font_size(10);
    let total_style = Style::new().with_color(btc_color).with_font_size(10);

    let data1 = vec![
        RowData::new("Fonte", table_header_style, Alignment::Left),
        RowData::new("Saldo", table_header_style, Alignment::Left),
        RowData::new("Custo", table_header_style, Alignment::Left),
        RowData::new("Preço", table_header_style, Alignment::Left),
    ];

    let row1 = row_table(data1);

    let data2 = vec![
        RowData::new("Comprado", table_value, Alignment::Left),
        RowData::new("0,5 BTC", table_value, Alignment::Left),
        RowData::new("R$0,0535", table_value, Alignment::Left),
        RowData::new("R$205.000", table_value, Alignment::Left),
    ];

    let row2 = row_table(data2);

    let data3 = vec![
        RowData::new("Vendido", table_value, Alignment::Left),
        RowData::new("-0,2 BTC", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
    ];

    let row3 = row_table(data3);

    let data4 = vec![
        RowData::new("Depositado", table_value, Alignment::Left),
        RowData::new("0,1 BTC", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
    ];

    let row4 = row_table(data4);

    let data5 = vec![
        RowData::new("Sacado", table_value, Alignment::Left),
        RowData::new("-0,1 BTC", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
    ];

    let row5 = row_table(data5);

    let total_data = vec![
        RowData::new("Total", total_style, Alignment::Left),
        RowData::new("0,3 BTC", total_style, Alignment::Left),
        RowData::new("R$0,0535", total_style, Alignment::Left),
        RowData::new("R$205.000", total_style, Alignment::Left),
    ];

    let total = row_table(total_data);

    let vertical_padding = Mm::from(3);
    let padding = Margins::trbl(zero, zero, vertical_padding, zero);
    let rows = vec![row1, row2, row3, row4, row5, total];
    let table = build_table(rows, padding);
    let table = table.styled(gray3);

    doc.push(table);

    let layout = gold_linear_layout(
        "89 - Outros criptoativos (PAX Gold - PAXG)",
        elements::Paragraph::default()
            .styled_string("Data:", gray4)
            .string(" 31/12/2021")
            .styled(style::Style::new().with_font_size(12))
            .styled(black),
    );

    let rows = vec![layout];
    let vertical_padding = Mm::from(5);
    let zero = Mm::from(0);
    let padding = Margins::trbl(vertical_padding, zero, vertical_padding, zero);

    let table = build_table(rows, padding);
    doc.push(table);

    let table_header_style = Style::new().with_color(gray3).with_font_size(10);
    let table_value = Style::new().with_color(black).with_font_size(10);
    let total_style = Style::new().with_color(gold_color).with_font_size(10);

    let data1 = vec![
        RowData::new("Fonte", table_header_style, Alignment::Left),
        RowData::new("Saldo", table_header_style, Alignment::Left),
        RowData::new("Custo", table_header_style, Alignment::Left),
        RowData::new("Preço", table_header_style, Alignment::Left),
    ];

    let row1 = row_table(data1);

    let data2 = vec![
        RowData::new("Comprado", table_value, Alignment::Left),
        RowData::new("0,5 PAXG", table_value, Alignment::Left),
        RowData::new("R$0,0535", table_value, Alignment::Left),
        RowData::new("R$205.000", table_value, Alignment::Left),
    ];

    let row2 = row_table(data2);

    let data3 = vec![
        RowData::new("Vendido", table_value, Alignment::Left),
        RowData::new("-0,2 PAXG", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
    ];

    let row3 = row_table(data3);

    let total_data = vec![
        RowData::new("Total", total_style, Alignment::Left),
        RowData::new("0,3 PAXG", total_style, Alignment::Left),
        RowData::new("R$0,0535", total_style, Alignment::Left),
        RowData::new("R$205.000", total_style, Alignment::Left),
    ];

    let total = row_table(total_data);

    let vertical_padding = Mm::from(3);
    let padding = Margins::trbl(zero, zero, vertical_padding, zero);
    let rows = vec![row1, row2, row3, total];
    let table = build_table(rows, padding);
    let table = table.styled(gray3);

    doc.push(table);

    doc.push(PageBreak::new());

    let layout = btc_linear_layout(
        "Bens e Direitos",
        "81 - Criptoativo Bitcoin - BTC",
        elements::Paragraph::default()
            .styled_string("Data:", gray4)
            .string(" 31/12/2020")
            .styled(style::Style::new().with_font_size(12))
            .styled(black),
    );

    let rows = vec![layout];
    let vertical_padding = Mm::from(5);
    let zero = Mm::from(0);
    let padding = Margins::trbl(vertical_padding, zero, vertical_padding, zero);

    let table = build_table(rows, padding);
    doc.push(table);

    let table_header_style = Style::new().with_color(gray3).with_font_size(10);
    let table_value = Style::new().with_color(black).with_font_size(10);
    let total_style = Style::new().with_color(btc_color).with_font_size(10);

    let data1 = vec![
        RowData::new("Fonte", table_header_style, Alignment::Left),
        RowData::new("Saldo", table_header_style, Alignment::Left),
        RowData::new("Custo", table_header_style, Alignment::Left),
        RowData::new("Preço", table_header_style, Alignment::Left),
    ];

    let row1 = row_table(data1);

    let data2 = vec![
        RowData::new("Comprado", table_value, Alignment::Left),
        RowData::new("0,5 BTC", table_value, Alignment::Left),
        RowData::new("R$0,0535", table_value, Alignment::Left),
        RowData::new("R$205.000", table_value, Alignment::Left),
    ];

    let row2 = row_table(data2);

    let data3 = vec![
        RowData::new("Vendido", table_value, Alignment::Left),
        RowData::new("-0,2 BTC", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
    ];

    let row3 = row_table(data3);

    let data4 = vec![
        RowData::new("Depositado", table_value, Alignment::Left),
        RowData::new("0,1 BTC", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
    ];

    let row4 = row_table(data4);

    let data5 = vec![
        RowData::new("Sacado", table_value, Alignment::Left),
        RowData::new("-0,1 BTC", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
    ];

    let row5 = row_table(data5);

    let total_data = vec![
        RowData::new("Total", total_style, Alignment::Left),
        RowData::new("0,3 BTC", total_style, Alignment::Left),
        RowData::new("R$0,0535", total_style, Alignment::Left),
        RowData::new("R$205.000", total_style, Alignment::Left),
    ];

    let total = row_table(total_data);

    let vertical_padding = Mm::from(3);
    let padding = Margins::trbl(zero, zero, vertical_padding, zero);
    let rows = vec![row1, row2, row3, row4, row5, total];
    let table = build_table(rows, padding);
    let table = table.styled(gray3);

    doc.push(table);

    let layout = gold_linear_layout(
        "89 - Outros criptoativos (PAX Gold - PAXG)",
        elements::Paragraph::default()
            .styled_string("Data:", gray4)
            .string(" 31/12/2020")
            .styled(style::Style::new().with_font_size(12))
            .styled(black),
    );

    let rows = vec![layout];
    let vertical_padding = Mm::from(5);
    let zero = Mm::from(0);
    let padding = Margins::trbl(vertical_padding, zero, vertical_padding, zero);

    let table = build_table(rows, padding);
    doc.push(table);

    let table_header_style = Style::new().with_color(gray3).with_font_size(10);
    let table_value = Style::new().with_color(black).with_font_size(10);
    let total_style = Style::new().with_color(gold_color).with_font_size(10);

    let data1 = vec![
        RowData::new("Fonte", table_header_style, Alignment::Left),
        RowData::new("Saldo", table_header_style, Alignment::Left),
        RowData::new("Custo", table_header_style, Alignment::Left),
        RowData::new("Preço", table_header_style, Alignment::Left),
    ];

    let row1 = row_table(data1);

    let data2 = vec![
        RowData::new("Comprado", table_value, Alignment::Left),
        RowData::new("0,5 PAXG", table_value, Alignment::Left),
        RowData::new("R$0,0535", table_value, Alignment::Left),
        RowData::new("R$205.000", table_value, Alignment::Left),
    ];

    let row2 = row_table(data2);

    let data3 = vec![
        RowData::new("Vendido", table_value, Alignment::Left),
        RowData::new("-0,2 PAXG", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
        RowData::new("N/A", table_value, Alignment::Left),
    ];

    let row3 = row_table(data3);

    let total_data = vec![
        RowData::new("Total", total_style, Alignment::Left),
        RowData::new("0,3 PAXG", total_style, Alignment::Left),
        RowData::new("R$0,0535", total_style, Alignment::Left),
        RowData::new("R$205.000", total_style, Alignment::Left),
    ];

    let total = row_table(total_data);

    let vertical_padding = Mm::from(3);
    let padding = Margins::trbl(zero, zero, vertical_padding, zero);
    let rows = vec![row1, row2, row3, total];
    let table = build_table(rows, padding);
    let table = table.styled(gray3);

    doc.push(table);

    doc.render_to_file("test_working.pdf")
        .expect("Failed to write output file");
}

fn main() {
    pdf()
}
