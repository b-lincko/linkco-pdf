use std::sync::Arc;

use super::*;
use crate::spool::{Duplex, Job, lp_args, parse_lpstat};

/// `n` pages of 200×300 (page 3 is landscape 300×200 when n ≥ 3); page i shows "(Page i+1)".
/// Page 1 carries a printable square comment, a non-printing note, and a stamp.
fn fixture(n: usize) -> Document {
    let mut objs: Vec<Vec<u8>> = vec![b"<< /Type /Catalog /Pages 2 0 R >>".to_vec()];
    let kids: Vec<String> = (0..n).map(|i| format!("{} 0 R", 6 + 2 * i)).collect();
    objs.push(format!("<< /Type /Pages /Kids [{}] /Count {n} /MediaBox [0 0 200 300] >>", kids.join(" ")).into_bytes());
    objs.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec());
    // 4: an appearance stream; 5: unused.
    let ap = b"0 0 1 rg 0 0 10 10 re f";
    objs.push(
        format!("<< /Type /XObject /Subtype /Form /BBox [0 0 10 10] /Length {} >>\nstream\n{}\nendstream", ap.len(), String::from_utf8_lossy(ap))
            .into_bytes(),
    );
    objs.push(b"null".to_vec());
    for i in 0..n {
        let annots = if i == 0 {
            " /Annots [<< /Type /Annot /Subtype /Square /F 4 /Rect [10 10 50 50] /AP << /N 4 0 R >> >> << /Type /Annot /Subtype /Text /F 0 /Rect [60 10 80 30] /AP << /N 4 0 R >> >> << /Type /Annot /Subtype /Stamp /F 4 /Rect [100 10 140 50] /AP << /N 4 0 R >> >>]"
        } else {
            ""
        };
        let mb = if i == 2 { " /MediaBox [0 0 300 200]" } else { "" };
        objs.push(
            format!("<< /Type /Page /Parent 2 0 R /Contents {} 0 R /Resources << /Font << /F1 3 0 R >> >>{mb}{annots} >>", 7 + 2 * i).into_bytes(),
        );
        let body = format!("BT /F1 24 Tf 20 150 Td (Page {}) Tj ET", i + 1);
        objs.push(format!("<< /Length {} >>\nstream\n{body}\nendstream", body.len()).into_bytes());
    }
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offs = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offs.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        out.extend_from_slice(o);
        out.extend_from_slice(b"\nendobj\n");
    }
    let x = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offs {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{x}\n%%EOF\n", objs.len() + 1).as_bytes());
    Document::open(Arc::new(out)).unwrap()
}

fn settings(pages: Vec<usize>, layout: Layout) -> Settings {
    Settings { pages, paper: (612.0, 792.0), layout, ..Settings::default() }
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

#[test]
fn page_selection() {
    let labels: Vec<String> = ["i", "ii", "1", "2", "A-1"].iter().map(|s| s.to_string()).collect();
    assert_eq!(select_pages(5, None, &labels, Subset::All, false).unwrap(), [0, 1, 2, 3, 4]);
    assert_eq!(select_pages(5, Some("1-2, 5"), &[], Subset::All, false).unwrap(), [0, 1, 4]);
    assert_eq!(select_pages(5, Some("4-"), &[], Subset::All, false).unwrap(), [3, 4]);
    assert_eq!(select_pages(5, Some("-2"), &[], Subset::All, false).unwrap(), [0, 1]);
    assert_eq!(select_pages(5, Some("ii-2, A-1"), &labels, Subset::All, false).unwrap(), [1, 2, 3, 4], "labels, even with a dash");
    assert_eq!(select_pages(5, None, &[], Subset::Even, true).unwrap(), [3, 1]);
    assert_eq!(select_pages(5, None, &[], Subset::Odd, false).unwrap(), [0, 2, 4]);
    assert!(matches!(select_pages(5, Some("7"), &[], Subset::All, false), Err(PrintError::Invalid(_))));
    assert!(matches!(select_pages(5, Some("x"), &[], Subset::All, false), Err(PrintError::Invalid(_))));
    assert_eq!(select_pages(1, None, &[], Subset::Even, false), Err(PrintError::NoPages));
    // An explicit list (selected thumbnails): positions, never labels, in the order given.
    assert_eq!(select_listed(5, &[1, 3], Subset::All, false).unwrap(), [1, 3]);
    assert_eq!(select_listed(5, &[0, 2, 4], Subset::Even, false).unwrap(), [2]);
    assert_eq!(select_listed(5, &[0, 2, 4], Subset::Odd, true).unwrap(), [4, 0]);
    assert!(matches!(select_listed(5, &[1, 5], Subset::All, false), Err(PrintError::Invalid(_))));
    assert!(matches!(select_listed(0, &[usize::MAX], Subset::All, false), Err(PrintError::Invalid(_))));
    assert_eq!(select_listed(5, &[], Subset::All, false), Err(PrintError::NoPages));
}

#[test]
fn size_modes() {
    let sizes = [(200.0, 300.0), (1000.0, 1500.0)];
    let fit = layout(&sizes, &settings(vec![0, 1], Layout::Size(SizeMode::Fit))).unwrap();
    let s0 = fit[0].placed[0].matrix.0[0];
    assert!(close(s0, 2.52), "min(576 / 200, 756 / 300): {s0}");
    let actual = layout(&sizes, &settings(vec![0], Layout::Size(SizeMode::Actual))).unwrap();
    assert_eq!(actual[0].placed[0].matrix.0, [1.0, 0.0, 0.0, 1.0, 206.0, 246.0], "centred at 100%");
    let shrink = layout(&sizes, &settings(vec![0, 1], Layout::Size(SizeMode::Shrink))).unwrap();
    assert_eq!(shrink[0].placed[0].matrix.0[0], 1.0, "small pages stay at 100%");
    assert!(shrink[1].placed[0].matrix.0[0] < 1.0, "big pages shrink");
    let custom = layout(&sizes, &settings(vec![0], Layout::Size(SizeMode::Custom(50.0)))).unwrap();
    assert_eq!(custom[0].placed[0].matrix.0[0], 0.5);
    // Auto orientation turns the sheet for landscape pages.
    let land = layout(&[(300.0, 200.0)], &settings(vec![0], Layout::Size(SizeMode::Fit))).unwrap();
    assert_eq!(land[0].size, (792.0, 612.0));
}

#[test]
fn multiple_pages_per_sheet() {
    let sizes = vec![(200.0, 300.0); 5];
    let sheets = layout(&sizes, &settings((0..5).collect(), Layout::multiple(4))).unwrap();
    assert_eq!(sheet_pages(&sheets), [vec![0, 1, 2, 3], vec![4]]);
    // Horizontal order: 0 top-left, 1 top-right, 2 bottom-left.
    let o = |k: usize| (sheets[0].placed[k].matrix.0[4], sheets[0].placed[k].matrix.0[5]);
    assert!(o(1).0 > o(0).0 && close(o(1).1, o(0).1) && o(2).1 < o(0).1 && close(o(2).0, o(0).0));
    // Two per sheet prints side by side on landscape paper.
    let two = layout(&sizes, &settings(vec![0, 1], Layout::multiple(2))).unwrap();
    assert_eq!(two[0].size, (792.0, 612.0));
    assert!(two[0].placed[1].matrix.0[4] > two[0].placed[0].matrix.0[4]);
    // Vertical order and borders.
    let v =
        layout(&sizes, &settings(vec![0, 1, 2], Layout::Multiple { cols: 2, rows: 2, order: PageOrder::Vertical, border: true, auto_rotate: false }))
            .unwrap();
    assert!(close(v[0].placed[1].matrix.0[4], v[0].placed[0].matrix.0[4]) && v[0].placed[1].matrix.0[5] < v[0].placed[0].matrix.0[5]);
    assert_eq!(v[0].borders.len(), 3);
    // Auto-rotate turns a landscape page in a portrait cell.
    let r = layout(&[(300.0, 200.0), (200.0, 300.0)], &settings(vec![0, 1], Layout::multiple(4))).unwrap();
    // The first (landscape) page sets a landscape sheet; the portrait page is turned to fit.
    assert_eq!(r[0].size, (792.0, 612.0));
    assert!(r[0].placed[0].matrix.0[0] > 0.0);
    assert_eq!(r[0].placed[1].matrix.0[0], 0.0, "rotated");
}

#[test]
fn booklets_pair_pages_for_folding() {
    let sizes = vec![(200.0, 300.0); 6];
    let sheets = layout(&sizes, &settings((0..6).collect(), Layout::Booklet { subset: BookletSubset::BothSides, binding: Binding::Left })).unwrap();
    // 6 pages pad to 8: sheet 1 front [8,1] → [blank, 0], back [1, 6]; sheet 2 front [5, 2], back [3, 4].
    assert_eq!(sheet_pages(&sheets), [vec![0], vec![1], vec![5, 2], vec![3, 4]]);
    assert!(sheets[0].placed[0].matrix.0[4] >= 395.9, "page 1 on the right half");
    assert_eq!(sheets[0].size, (792.0, 612.0));
    let front = layout(&sizes, &settings((0..6).collect(), Layout::Booklet { subset: BookletSubset::FrontOnly, binding: Binding::Right })).unwrap();
    assert_eq!(sheet_pages(&front), [vec![0], vec![2, 5]], "right binding mirrors the spread");
}

#[test]
fn posters_tile_with_overlap() {
    // A 200×300 page at 400% = 800×1200 on Letter with 18 pt margins (576×756 tiles), overlap 36.
    let sheets = layout(&[(200.0, 300.0)], &settings(vec![0], Layout::Poster { scale: 400.0, overlap: 36.0, cut_marks: true })).unwrap();
    assert_eq!(sheets.len(), 4, "2 × 2 tiles");
    assert_eq!(sheets[0].lines.len(), 8, "cut marks");
    let c0 = sheets[0].placed[0].clip;
    let c1 = sheets[1].placed[0].clip;
    assert!(close(c0[2] - c1[0], 36.0 / 4.0), "neighbouring tiles share the overlap");
    assert!(layout(&[(200.0, 300.0)], &settings(vec![0], Layout::Poster { scale: 400.0, overlap: 400.0, cut_marks: false })).is_err());
}

/// Issue #1335: a scale so large that the tile counts saturate is refused with the sheet-limit
/// error, not an overflow in counting the sheets.
#[test]
fn huge_posters_are_refused_at_the_sheet_limit() {
    let refused = Err(PrintError::Invalid("the poster would need more than 1024 sheets".into()));
    for scale in [100_000.0, 1e22, f64::MAX] {
        let poster = settings(vec![0], Layout::Poster { scale, overlap: 18.0, cut_marks: true });
        assert_eq!(layout(&[(200.0, 300.0)], &poster), refused, "scale {scale}");
    }
}

#[test]
fn imposed_pdf_has_the_sheets_and_honours_comments_and_forms() {
    let doc = fixture(3);
    let out = impose(&doc, &settings(vec![0, 1, 2], Layout::multiple(2))).unwrap();
    let printed = Document::open(Arc::new(out.clone())).unwrap();
    let pages = pdfcraft_model::pages(&printed);
    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].crop(&printed), [0.0, 0.0, 792.0, 612.0]);
    // The source pages are form XObjects holding their content.
    let streams = |d: &Document| -> Vec<String> {
        d.object_numbers()
            .into_iter()
            .filter_map(|n| match &*d.get(ObjRef::new(n, d.generation(n))) {
                Object::Stream(s) => s.decoded().ok().map(|b| String::from_utf8_lossy(&b).into_owned()),
                _ => None,
            })
            .collect()
    };
    let all = streams(&printed).join("\n");
    assert!(all.contains("(Page 1)") && all.contains("(Page 3)"));
    // Markups: the printable square and the stamp, not the note without the Print flag.
    let page1 = streams(&printed).into_iter().find(|s| s.contains("(Page 1)")).unwrap();
    assert_eq!(page1.matches(" Do Q").count(), 2, "{page1}");
    let doc_only = impose(&doc, &Settings { content: Content::Document, ..settings(vec![0], Layout::Size(SizeMode::Fit)) }).unwrap();
    let p = streams(&Document::open(Arc::new(doc_only)).unwrap()).into_iter().find(|s| s.contains("(Page 1)")).unwrap();
    assert_eq!(p.matches(" Do Q").count(), 0);
    let stamps = impose(&doc, &Settings { content: Content::DocumentAndStamps, ..settings(vec![0], Layout::Size(SizeMode::Fit)) }).unwrap();
    let p = streams(&Document::open(Arc::new(stamps)).unwrap()).into_iter().find(|s| s.contains("(Page 1)")).unwrap();
    assert_eq!(p.matches(" Do Q").count(), 1);
    let fields = impose(&doc, &Settings { content: Content::FormFieldsOnly, ..settings(vec![1], Layout::Size(SizeMode::Fit)) }).unwrap();
    assert!(!streams(&Document::open(Arc::new(fields)).unwrap()).join("").contains("(Page 2)"));
    // The output is a fresh file: the unused source pages are gone.
    let one = impose(&doc, &settings(vec![1], Layout::Size(SizeMode::Fit))).unwrap();
    let s = streams(&Document::open(Arc::new(one)).unwrap()).join("\n");
    assert!(s.contains("(Page 2)") && !s.contains("(Page 1)"));
}

#[test]
fn spooler_arguments_and_printer_list() {
    let out = "printer Office_Laser is idle.  enabled since Thu Oct  1 09:00:00 2026\nprinter Label_Writer disabled since …\nsystem default destination: Office_Laser\n";
    assert_eq!(
        parse_lpstat(out),
        [spool::Printer { name: "Office_Laser".into(), default: true }, spool::Printer { name: "Label_Writer".into(), default: false }]
    );
    assert!(parse_lpstat("lpstat: No destinations added.\nno system default destination\n").is_empty());
    let job = Job {
        printer: Some("Office_Laser".into()),
        copies: 3,
        collate: false,
        duplex: Duplex::LongEdge,
        grayscale: true,
        title: "memo.pdf".into(),
        options: Vec::new(),
    };
    assert_eq!(
        lp_args(&job).join(" "),
        "-d Office_Laser -n 3 -t memo.pdf -o collate=false -o sides=two-sided-long-edge -o print-color-mode=monochrome -o fit-to-page=false"
    );
    assert_eq!(lp_args(&Job::default())[0], "-n", "no -d: the default printer");
}

#[test]
fn windows_printer_enumeration_and_status_parsing() {
    use crate::spool::{PrinterStatus, apply_grayscale_rgba, parse_lpstat_detailed, parse_windows_printers_tsv, parse_windows_reg_printers};

    let tsv = "\
Microsoft Print to PDF\tTrue\tFalse\t3\t0\tPORTPROMPT:\tMicrosoft Print To PDF
\\\\doha-print01\\Linkco_HQ_Ricoh\tFalse\tFalse\t4\t0\tIP_10.10.20.15\tRicoh PCL6 UniversalDriver
HP_LaserJet_Warehouse\tFalse\tTrue\t3\t7\tUSB001\tHP Universal Printing PCL 6
Canon_Plotter_A0\tFalse\tFalse\t6\t8\tWSD-1234\tCanon Driver
Brother_Label\tFalse\tFalse\t3\t9\tUSB002\tBrother Driver
";
    let list = parse_windows_printers_tsv(tsv);
    assert_eq!(list.len(), 5);
    assert_eq!(list[0].name, "Microsoft Print to PDF");
    assert!(list[0].default);
    assert_eq!(list[0].status, PrinterStatus::Ready);
    assert!(!list[0].network);

    assert_eq!(list[1].name, r"\\doha-print01\Linkco_HQ_Ricoh");
    assert!(!list[1].default);
    assert_eq!(list[1].status, PrinterStatus::Printing);
    assert!(list[1].network);

    assert_eq!(list[2].name, "HP_LaserJet_Warehouse");
    assert_eq!(list[2].status, PrinterStatus::Offline);
    assert!(list[2].status.is_offline());
    assert!(!list[2].status.is_available());

    assert_eq!(list[3].name, "Canon_Plotter_A0");
    assert_eq!(list[3].status, PrinterStatus::Paused);

    assert_eq!(list[4].name, "Brother_Label");
    assert_eq!(list[4].status, PrinterStatus::Error);

    assert!(list[3].network, "a WSD port is a network printer");

    // Network printers the port names alone show: Standard TCP/IP ports named after the address or
    // host (Windows' default naming), IPP and LPR; and Win32_Printer's own Network flag (8th column).
    let tsv = "\
Office_TCP\tFalse\tFalse\t3\t0\t192.168.1.20\tHP Universal\tFalse
Office_TCP_Second\tFalse\tFalse\t3\t0\t192.168.1.20_1\tHP Universal\tFalse
Plotter_Host\tFalse\tFalse\t3\t0\tplotter.linkco.local:9100\tHP DesignJet\tFalse
Cloud_IPP\tFalse\tFalse\t3\t0\thttp://print.linkco.local:631/ipp/print\tMicrosoft IPP Class Driver\tFalse
Server_Queue\tFalse\tFalse\t3\t0\tNe03:\tKyocera\tTrue
Desk_USB\tFalse\tFalse\t3\t0\tUSB001\tEpson\tFalse
To_File\tFalse\tFalse\t3\t0\tC:\\out\\print.prn\tGeneric\tFalse
";
    let network: Vec<(String, bool)> = parse_windows_printers_tsv(tsv).into_iter().map(|p| (p.name, p.network)).collect();
    let expect = [
        ("Office_TCP", true),
        ("Office_TCP_Second", true),
        ("Plotter_Host", true),
        ("Cloud_IPP", true),
        ("Server_Queue", true),
        ("Desk_USB", false),
        ("To_File", false),
    ];
    assert_eq!(network, expect.map(|(n, b)| (n.to_string(), b)));

    // Registry fallback parsing (HKCU\Software\Microsoft\Windows NT\CurrentVersion\Devices).
    let reg_devices = "\
HKEY_CURRENT_USER\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Devices
    Microsoft Print to PDF    REG_SZ    winspool,Ne01:
    \\\\server\\Floor2_Printer    REG_SZ    winspool,Ne02:
";
    let reg_default = "\
HKEY_CURRENT_USER\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Windows
    Device    REG_SZ    \\\\server\\Floor2_Printer,winspool,Ne02:
";
    let reg_list = parse_windows_reg_printers(reg_devices, reg_default);
    assert_eq!(reg_list.len(), 2);
    assert!(!reg_list[0].default);
    assert!(reg_list[1].default);
    assert!(reg_list[1].network);

    // Detailed CUPS lpstat status parsing.
    let lp = "printer Office_Laser is idle.  enabled since Thu Oct  1 09:00:00 2026\nprinter Label_Writer disabled since …\nsystem default destination: Office_Laser\n";
    let lp_det = parse_lpstat_detailed(lp);
    assert_eq!(lp_det[0].status, PrinterStatus::Ready);
    assert_eq!(lp_det[1].status, PrinterStatus::Paused);

    // Grayscale RGBA conversion preserves alpha and neutralises colour channels.
    let mut px = [255u8, 0, 0, 200, 0, 255, 0, 255];
    apply_grayscale_rgba(&mut px);
    assert_eq!(px[0], px[1]);
    assert_eq!(px[1], px[2]);
    assert_eq!(px[3], 200);
    assert_eq!(px[4], px[5]);
    assert_eq!(px[5], px[6]);
    assert_eq!(px[7], 255);
}

#[test]
fn lpstat_output_is_untranslated() {
    // A localized lpstat (here Polish) is unreadable to parse_lpstat...
    assert!(parse_lpstat("drukarka Office_Laser jest bezczynna.\ndomyślny cel systemowy: Office_Laser\n").is_empty());
    // ...so the command must force the C locale, including the SOFTWARE switch macOS CUPS needs.
    let cmd = spool::lpstat_command();
    let envs: Vec<_> = cmd.get_envs().map(|(k, v)| (k.to_string_lossy().into_owned(), v.map(|v| v.to_string_lossy().into_owned()))).collect();
    for key in ["LC_ALL", "LANG"] {
        assert!(envs.contains(&(key.into(), Some("C".into()))), "{key}=C missing: {envs:?}");
    }
    assert!(envs.iter().any(|(k, v)| k == "SOFTWARE" && v.as_deref().is_some_and(|v| !v.is_empty())), "SOFTWARE missing: {envs:?}");
}

#[test]
fn driverless_printers_join_the_list_without_a_queue() {
    // A driverless network printer (IPP, discovered on the LAN) has no queue, so `lpstat -p`
    // never lists it — yet GTK's print dialog shows it and `lp` prints to it through a temporary
    // queue. Only `lpstat -e` names it, and the Print dialog used to show Save as PDF alone.
    let queues = "printer Office_Laser is idle.  enabled since Thu Oct  1 09:00:00 2026\nsystem default destination: Office_Laser\n";
    assert_eq!(
        spool::printers_parsed(queues, Some("Basement_Color\nOffice_Laser\n")),
        [spool::Printer { name: "Office_Laser".into(), default: true }, spool::Printer { name: "Basement_Color".into(), default: false },],
        "queues first, then the driverless destination; the queue listed twice is not doubled"
    );
    // An old spooler without `-e` (CUPS < 1.7) keeps its queues; only the driverless names are lost.
    assert_eq!(spool::printers_parsed(queues, None), [spool::Printer { name: "Office_Laser".into(), default: true }]);
    // The default can itself be driverless (`lpoptions -d` names a destination no queue exists
    // for): it keeps its marker, so the dialog preselects the user's default, not the first queue.
    assert_eq!(
        spool::printers_parsed("system default destination: Basement_Color\n", Some("Basement_Color\n")),
        [spool::Printer { name: "Basement_Color".into(), default: true }],
        "a default destination without a queue is still the default"
    );
    assert!(spool::parse_lpstat_e("\n \n").is_empty(), "no destinations is empty, not [\"\"]");
    assert_eq!(spool::parse_lpstat_e("A\r\nB \n"), ["A".to_string(), "B".to_string()], "CRLF and stray spaces are trimmed");
}

fn cut_stack(cols: usize, rows: usize) -> Layout {
    Layout::Multiple { cols, rows, order: PageOrder::CutStack, border: false, auto_rotate: false }
}

#[test]
fn cut_stack_keeps_piles_in_order_and_blanks_in_place() {
    let sizes = vec![(200.0, 300.0); 10];
    let sheets = layout(&sizes, &settings((0..10).collect(), cut_stack(2, 2))).unwrap();
    assert_eq!(sheet_pages(&sheets), [vec![0, 3, 6, 9], vec![1, 4, 7], vec![2, 5, 8]]);
    for s in &sheets {
        assert_eq!(s.lines, sheets[0].lines, "identical cuts even with blank cells");
        assert_eq!(s.lines.len(), 4);
        for (pl, first) in s.placed.iter().zip(&sheets[0].placed) {
            assert_eq!(pl.matrix, first.matrix, "every pile stays in its cell");
        }
    }
    assert!(sheets[0].placed[1].matrix.0[4] > sheets[0].placed[0].matrix.0[4]);
    assert!(sheets[0].placed[2].matrix.0[5] < sheets[0].placed[0].matrix.0[5]);
    // Selection and reverse order are preserved, including repeated source pages.
    let selected = vec![9, 5, 5, 2, 0];
    let sheets = layout(&sizes, &settings(selected, cut_stack(2, 2))).unwrap();
    assert_eq!(sheet_pages(&sheets), [vec![9, 5, 0], vec![5, 2]]);
    assert_eq!(sheets[1].placed[1].matrix, sheets[0].placed[1].matrix);
}

#[test]
fn cutting_and_stacking_recovers_every_selected_page() {
    // Simulate the actual operation using cell positions, not the imposition formula.
    for (cols, rows) in [(1, 1), (1, 2), (2, 2), (2, 3), (3, 3), (4, 4)] {
        for orientation in [Orientation::Auto, Orientation::Portrait, Orientation::Landscape] {
            for n in 1..=37 {
                let sizes = vec![(200.0, 300.0); n];
                let selected: Vec<usize> = (0..n).rev().collect();
                let s = Settings { orientation, ..settings(selected.clone(), cut_stack(cols, rows)) };
                let sheets = layout(&sizes, &s).unwrap();
                let mut piles: std::collections::BTreeMap<(i64, i64), Vec<usize>> = Default::default();
                for sheet in &sheets {
                    for pl in &sheet.placed {
                        // Equal source sizes, so the page origins identify row/column.
                        let [_, _, _, _, x, y] = pl.matrix.0;
                        piles.entry((-(y * 100.0).round() as i64, (x * 100.0).round() as i64)).or_default().push(pl.page);
                    }
                }
                let restacked: Vec<usize> = piles.into_values().flatten().collect();
                assert_eq!(restacked, selected, "{cols}x{rows}, {n} pages, {orientation:?}");
            }
        }
    }
}

#[test]
fn multiple_rejects_hostile_grids_without_panicking() {
    let sizes = [(200.0, 300.0)];
    for (cols, rows) in [(usize::MAX, 2), (2, usize::MAX), (0, 2), (1, 0), (257, 1)] {
        let mode = Layout::Multiple { cols, rows, order: PageOrder::Horizontal, border: false, auto_rotate: false };
        assert!(matches!(layout(&sizes, &settings(vec![0], mode)), Err(PrintError::Invalid(_))));
    }
    let tiny = Settings { paper: (72.0, 72.0), ..settings(vec![0], cut_stack(16, 16)) };
    assert!(matches!(layout(&sizes, &tiny), Err(PrintError::Invalid(_))));
    for size in [(0.0, 300.0), (200.0, f64::NAN), (f64::INFINITY, 300.0), (f64::from_bits(1), f64::from_bits(1))] {
        assert!(matches!(layout(&[size], &settings(vec![0], cut_stack(2, 2))), Err(PrintError::Invalid(_))));
    }
}

/// Set on the child process when a test runs this test binary as a stand-in for `lp`.
const STAND_IN_LP: &str = "PDFCRAFT_STAND_IN_LP";

/// Not a test of its own: [`stand_in_lp`] re-runs this binary with only this test selected, and it
/// then plays `lp`. It records its arguments and stdin in the folder named by [`STAND_IN_LP`], or,
/// when the folder is `refuse`, exits without reading stdin, the way `lp` refuses an unknown
/// printer.
#[test]
fn stand_in_for_lp() {
    let Some(record) = std::env::var_os(STAND_IN_LP) else { return };
    if record == "refuse" {
        eprintln!("lp: The printer or class does not exist.");
        std::process::exit(1);
    }
    let record = std::path::PathBuf::from(record);
    let args: Vec<String> = std::env::args().skip_while(|a| a != "--").skip(1).collect();
    let mut stdin = Vec::new();
    std::io::Read::read_to_end(&mut std::io::stdin(), &mut stdin).unwrap();
    std::fs::write(record.join("args"), args.join(" ")).unwrap();
    std::fs::write(record.join("stdin"), stdin).unwrap();
}

/// A spooler command that runs [`stand_in_for_lp`] with `record` (see there).
fn stand_in_lp(record: &std::ffi::OsStr) -> std::process::Command {
    let mut c = std::process::Command::new(std::env::current_exe().unwrap());
    c.args(["--exact", "tests::stand_in_for_lp", "--nocapture", "--"]).env(STAND_IN_LP, record);
    c
}

#[test]
fn print_jobs_reach_lp_on_stdin_never_through_a_shared_temp_folder() {
    // The job used to be written to `<temp>/pdfcraft-print-<pid>/job-<time>.pdf` and handed to `lp`
    // by name. In a shared /tmp another local user can predict that folder, create it first (or
    // plant a symlink there) and so read every printed document or swap the file before `lp`
    // reads it. Piping the job to `lp` leaves nothing on disk.
    static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let record =
        std::env::temp_dir().join(format!("pdfcraft-print-test-{}-{}", std::process::id(), N.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
    std::fs::create_dir(&record).unwrap();
    // Bigger than any pipe buffer, so the spooler must read while the job is still being written.
    let pdf: Vec<u8> = b"%PDF-1.7 synthetic print job\n".iter().copied().cycle().take(3 << 20).collect();
    let job = Job { title: "memo.pdf".into(), ..Job::default() };
    let sent = spool::submit_via(stand_in_lp(record.as_os_str()), &pdf, &job);
    let args = std::fs::read_to_string(record.join("args"));
    let stdin = std::fs::read(record.join("stdin"));
    let _ = std::fs::remove_dir_all(&record);
    assert!(sent.is_ok(), "{sent:?}");
    assert_eq!(
        args.unwrap(),
        "-n 1 -t memo.pdf -o collate=true -o sides=one-sided -o fit-to-page=false",
        "no file argument: lp reads the job from stdin"
    );
    assert!(stdin.unwrap() == pdf, "lp receives the whole job on stdin");
    let predictable = std::env::temp_dir().join(format!("pdfcraft-print-{}", std::process::id()));
    assert!(!predictable.exists(), "nothing is created at {predictable:?}, a name other local users can predict");
}

#[test]
fn a_refused_print_job_reports_the_spoolers_message() {
    // `lp` refuses an unknown printer without reading the job; the user sees why, not a broken pipe.
    let pdf = vec![b'%'; 3 << 20];
    let err = spool::submit_via(stand_in_lp("refuse".as_ref()), &pdf, &Job::default());
    assert_eq!(err, Err(PrintError::Spool("lp: The printer or class does not exist.".into())));
}

#[test]
fn the_spoolers_reply_drops_the_file_count_of_a_job_sent_on_stdin() {
    assert_eq!(
        spool::job_message(
            b"request id is Office_Laser-12 (0 file(s))
"
        ),
        "request id is Office_Laser-12"
    );
    assert_eq!(
        spool::job_message(
            b"request id is Office_Laser-13 (1 file(s))
"
        ),
        "request id is Office_Laser-13 (1 file(s))"
    );
    assert_eq!(spool::job_message(b""), "");
}

/// Windows: the printer list comes from the spooler (`Win32_Printer`, then .NET, then the
/// registry). A bare machine may have no printer; the list that exists has real names and at most
/// one default, and the detailed list the Print dialog uses names the same printers.
#[cfg(windows)]
#[test]
fn windows_lists_the_printers_the_system_knows() {
    let printers = spool::printers();
    assert!(printers.iter().all(|p| !p.name.trim().is_empty()), "{printers:?}");
    assert!(printers.iter().filter(|p| p.default).count() <= 1, "{printers:?}");
    let detailed = spool::printers_detailed();
    assert!(detailed.iter().filter(|p| p.default).count() <= 1, "{detailed:?}");
}

/// A PPD shaped like a Fiery's: installable options, multi-line PostScript in the choices, Latin-1
/// labels, options the Print dialog sets itself.
const FIERY_PPD: &[u8] = b"*PPD-Adobe: \"4.3\"\n\
*LanguageEncoding: ISOLatin1\n\
*OpenGroup: InstallableOptions/Installable Options\n\
*OpenUI *EFFinisher/Finisher option: PickOne\n\
*DefaultEFFinisher: False\n\
*EFFinisher False/Not installed: \"\"\n\
*EFFinisher SingleStapler/Single stapler: \"\"\n\
*CloseUI: *EFFinisher\n\
*CloseGroup: InstallableOptions\n\
*OpenGroup: FPPaperSource/Media\n\
*OpenUI *InputSlot/Paper tray: PickOne\n\
*OrderDependency: 20.0 AnySetup *InputSlot\n\
*DefaultInputSlot: AutoSelect\n\
*InputSlot AutoSelect/Auto tray select: \"\n\
userdict /XJXEFIsetpageproperties known\n\
{ << /XJXsettrayselV2 [ 7 ] >> XJXEFIsetpageproperties } if\"\n\
*End\n\
*InputSlot ManualFeed/Bypass tray: \"\n\
{ pop 2 XJXsettrayselV2 } if\"\n\
*End\n\
*InputSlot Tray2/Tray 2: \"\"\n\
*fr.InputSlot Tray2/Bac 2: \"\"\n\
*CloseUI: *InputSlot\n\
*OpenUI *EFMediaType/Paper type: PickOne\n\
*DefaultEFMediaType: Plain\n\
*EFMediaType Plain/Plain: \"\"\n\
*EFMediaType Heavy1/Thick 1 (106\xad163 g/m\xb2): \"\"\n\
*CloseUI: *EFMediaType\n\
*CloseGroup: FPPaperSource\n\
*OpenUI *PageSize/Page size: PickOne\n\
*DefaultPageSize: A4\n\
*PageSize A4/A4: \"\"\n\
*CloseUI: *PageSize\n\
*OpenUI *EFRaster/Print queue action: PickOne\n\
*DefaultEFRaster: Bogus\n\
*EFRaster False/Print: \"\"\n\
*EFRaster Hold: \"\"\n\
*CloseUI: *EFRaster\n";

#[test]
fn printer_options_come_from_the_ppd() {
    let options = spool::parse_ppd(&spool::ppd_text(FIERY_PPD));
    let keys: Vec<&str> = options.iter().map(|o| o.key.as_str()).collect();
    assert_eq!(keys, ["InputSlot", "EFMediaType", "EFRaster"], "no installable options, no PageSize");
    let tray = &options[0];
    assert_eq!((tray.label.as_str(), tray.group.as_str(), tray.default.as_str()), ("Paper tray", "Media", "AutoSelect"));
    assert_eq!(
        tray.choices,
        [("AutoSelect".into(), "Auto tray select".into()), ("ManualFeed".into(), "Bypass tray".into()), ("Tray2".into(), "Tray 2".into())],
        "PostScript lines and translations (*fr.InputSlot) are not choices"
    );
    assert_eq!(options[1].choices[1].1, "Thick 1 (106\u{ad}163 g/m²)", "Latin-1 labels");
    // A default that isn't one of the choices falls back to the first; a choice without a label shows its keyword.
    assert_eq!((options[2].default.as_str(), options[2].choices[1].1.as_str()), ("False", "Hold"));
    assert!(spool::parse_ppd("*OpenUI *Broken\n*Broken A/B\n").is_empty(), "unclosed or malformed blocks are skipped");
}

#[test]
fn printer_option_defaults_and_job_arguments() {
    let current =
        spool::parse_lpoptions("InputSlot/Paper tray: AutoSelect *Tray2 ManualFeed\nEFRaster/Print queue action: *False Hold\nbroken line\n");
    assert_eq!(current, [("InputSlot".to_string(), "Tray2".to_string()), ("EFRaster".to_string(), "False".to_string())]);
    let job = Job {
        options: vec![
            ("InputSlot".into(), "Tray2".into()),
            ("EFMediaType".into(), "Heavy1".into()),
            ("Duplex".into(), "DuplexTumble".into()),
            ("Bad Key".into(), "x".into()),
            ("EFRaster".into(), "a=b".into()),
        ],
        ..Job::default()
    };
    let args = lp_args(&job).join(" ");
    assert!(args.ends_with("-o fit-to-page=false -o InputSlot=Tray2 -o EFMediaType=Heavy1"), "{args}");
    assert!(spool::printer_options("../../etc/passwd").is_empty(), "not a queue name");
}

#[cfg(all(unix, not(target_arch = "wasm32")))]
mod cups_options {
    use crate::spool;

    const RECORD: &str = "PDFCRAFT_LPOPTIONS_TEST_RECORD";
    const ANSWER: &str = "PDFCRAFT_LPOPTIONS_TEST_ANSWER";

    /// The child process stands in for lpoptions, recording its real arguments.
    #[test]
    fn stand_in_for_lpoptions() {
        let Some(record) = std::env::var_os(RECORD) else { return };
        let args: Vec<String> = std::env::args().skip_while(|a| a != "--").skip(1).collect();
        std::fs::write(std::path::Path::new(&record).join("args"), args.join("\n")).unwrap();
        println!("{}", std::env::var(ANSWER).unwrap());
    }

    fn command(record: &std::path::Path, answer: &str) -> std::process::Command {
        let mut c = std::process::Command::new(std::env::current_exe().unwrap());
        // Serial libtest otherwise prefixes the first output line with the test name,
        // corrupting the mocked lpoptions key. Quiet mode leaves the answer on its own line.
        c.args(["--quiet", "--exact", "tests::cups_options::stand_in_for_lpoptions", "--nocapture", "--"]).env(RECORD, record).env(ANSWER, answer);
        c
    }

    fn folder() -> std::path::PathBuf {
        use std::io::Read as _;
        use std::os::unix::fs::DirBuilderExt as _;
        let mut random = std::fs::File::open("/dev/urandom").unwrap();
        for _ in 0..32 {
            let mut nonce = [0; 16];
            random.read_exact(&mut nonce).unwrap();
            let path = std::env::temp_dir().join(format!("pdfcraft-cups-options-{:032x}", u128::from_be_bytes(nonce)));
            // Atomic creation, private permissions from the outset; never reuse an existing
            // path (including a symlink) in the shared temporary directory.
            match std::fs::DirBuilder::new().mode(0o700).create(&path) {
                Ok(()) => return path,
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("create private CUPS test directory: {e}"),
            }
        }
        panic!("could not create a unique CUPS test directory");
    }

    #[test]
    fn lpoptions_child_output_is_independent_of_harness_threads() {
        let dir = folder();
        for threads in ["1", "2"] {
            let output =
                command(&dir, "InputSlot/Paper tray: *Tray2").env("RUST_TEST_THREADS", threads).args(["-p", "Office/duplex", "-l"]).output().unwrap();
            assert!(output.status.success(), "threads={threads}: {output:?}");
            let stdout = String::from_utf8(output.stdout).unwrap();
            assert_eq!(spool::parse_lpoptions(&stdout), [("InputSlot".into(), "Tray2".into())], "threads={threads}: {stdout}");
            assert_eq!(std::fs::read_to_string(dir.join("args")).unwrap(), "-p\nOffice/duplex\n-l");
        }
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn instances_read_the_queue_ppd_and_their_own_defaults() {
        use std::os::unix::fs::PermissionsExt as _;
        let dir = folder();
        assert_eq!(std::fs::metadata(&dir).unwrap().permissions().mode() & 0o777, 0o700);
        // One synthetic queue PPD; there is deliberately no Office/ subdirectory.
        std::fs::write(
            dir.join("Office.ppd"),
            "*OpenUI *InputSlot/Paper tray: PickOne\n*DefaultInputSlot: Auto\n*InputSlot Auto/Automatic: \"\"\n*InputSlot Tray2/Tray 2: \"\"\n*CloseUI: *InputSlot\n",
        )
        .unwrap();
        let boundary_queue = "Q".repeat(64);
        let boundary_instance = format!("Office/{}", "I".repeat(64));
        std::fs::copy(dir.join("Office.ppd"), dir.join(format!("{boundary_queue}.ppd"))).unwrap();
        for (destination, current, expected) in [
            ("Office", "Auto", "Auto"),
            ("Office/duplex", "Tray2", "Tray2"),
            ("Office/draft", "Auto", "Auto"),
            ("Office/unknown-default", "Bogus", "Auto"),
            (boundary_queue.as_str(), "Tray2", "Tray2"),
            (boundary_instance.as_str(), "Tray2", "Tray2"),
        ] {
            let options = spool::printer_options_via(destination, &dir, command(&dir, &format!("InputSlot/Paper tray: *{current}")));
            assert_eq!(options.len(), 1, "{destination}: the queue's PPD must be read");
            assert_eq!(options[0].default, expected, "{destination}: only valid instance choices override the PPD default");
            assert_eq!(options[0].choices, [("Auto".into(), "Automatic".into()), ("Tray2".into(), "Tray 2".into())]);
            assert_eq!(std::fs::read_to_string(dir.join("args")).unwrap(), format!("-p\n{destination}\n-l"));
        }
        let job = spool::Job { printer: Some("Office/duplex".into()), ..Default::default() };
        assert_eq!(&spool::lp_args(&job)[..2], ["-d", "Office/duplex"]);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn invalid_destinations_are_rejected_before_running_lpoptions() {
        let dir = folder();
        let long_queue = "Q".repeat(65);
        let long_queue_instance = format!("{long_queue}/duplex");
        let long_instance = format!("Office/{}", "I".repeat(65));
        for destination in [
            "",
            ".",
            "..",
            "/Office",
            "Office/",
            "Office/a/b",
            "../../etc/passwd",
            "./duplex",
            "../duplex",
            "Office/.",
            "Office/..",
            "Office\\duplex",
            "Office/\\duplex",
            "Office/with space",
            "Office/line\nbreak",
            "Office/quote\"",
            "Office/equals=",
            "Office/a:b",
            "Office/\0",
            long_queue.as_str(),
            long_queue_instance.as_str(),
            long_instance.as_str(),
        ] {
            assert!(spool::printer_options_via(destination, &dir, command(&dir, "InputSlot: *Auto")).is_empty(), "{destination:?}");
            assert!(!dir.join("args").exists(), "{destination:?}: rejected before spawning");
        }
        std::fs::remove_dir_all(dir).unwrap();
    }
}

#[test]
fn a_scaled_appearance_is_not_scaled_twice() {
    let doc = fixture(1);
    let nums = |v: [f64; 6]| Object::Array(v.iter().map(|x| Object::Real(*x)).collect());
    let mut form = Dict::new();
    form.set(b"BBox".to_vec(), Object::Array([0.0, 0.0, 10.0, 10.0].iter().map(|x| Object::Real(*x)).collect()));
    form.set(b"Matrix".to_vec(), nums([2.0, 0.0, 0.0, 2.0, 0.0, 0.0]));
    // The form's own /Matrix doubles it to 20x20 when drawn; the placement only has to map 20x20 onto the 40x40 rect.
    let m = appearance_matrix(&doc, &form, [10.0, 10.0, 50.0, 50.0]).unwrap();
    assert!(close(m.0[0], 2.0) && close(m.0[3], 2.0), "{:?}", m.0);
    assert!(close(m.0[4], 10.0) && close(m.0[5], 10.0), "{:?}", m.0);
}

#[test]
fn network_printer_ports_are_told_from_local_ones() {
    use crate::spool::is_network_port;
    for port in [
        r"\\print01\Floor2",
        "IP_10.10.20.15",
        "10.10.20.15",
        "10.10.20.15_2",
        "10.10.20.15:9100",
        "printer.corp.local",
        "printer-2.corp.local_1",
        "WSD-6a1f0e3c-1d2b-4c5d-9e8f-0a1b2c3d4e5f",
        "http://print.corp.local:631/ipp/print",
        "https://print.corp.local/ipp",
        "IPP_print.corp.local",
        "LPR:10.0.0.9:queue",
    ] {
        assert!(is_network_port(port), "{port} is a network port");
    }
    for port in [
        "",
        "USB001",
        "LPT1:",
        "COM3:",
        "FILE:",
        "PORTPROMPT:",
        "nul:",
        "XPSPort:",
        "SHRFAX:",
        "Ne01:",
        r"C:\out\print.prn",
        "print.prn",
        "out.pdf",
        "10.0.0",
        "_1",
    ] {
        assert!(!is_network_port(port), "{port:?} is a local port");
    }
}

#[test]
fn architectural_sheets_preserve_full_scale() {
    let index = matching_paper((2592.0, 1728.0)).unwrap();
    assert_eq!(PAPERS[index].0, "ARCH D (24 x 36 in)");
    assert_eq!(matching_paper((1728.0, 2592.0)), Some(index));
    assert_eq!(matching_paper((f64::NAN, 2592.0)), None);
    let s = Settings { pages: vec![0], paper: PAPERS[index].1, layout: Layout::Size(SizeMode::Actual), ..Settings::default() };
    let sheets = layout(&[(2592.0, 1728.0)], &s).unwrap();
    assert_eq!(sheets[0].size, (2592.0, 1728.0));
    assert_eq!(sheets[0].placed[0].matrix.0, [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    let fit = layout(&[(2592.0, 1728.0)], &Settings { layout: Layout::Size(SizeMode::Fit), ..s }).unwrap();
    assert!(fit[0].placed[0].matrix.0[0] < 1.0, "Fit adds margins and must not masquerade as full scale");
}

#[test]
fn paper_matching_accepts_rounded_metric_sizes_but_not_custom_or_invalid_sizes() {
    for (i, (_, (w, h))) in PAPERS.iter().enumerate() {
        assert_eq!(matching_paper((*w, *h)), Some(i));
        assert_eq!(matching_paper((*h, *w)), Some(i));
        assert_eq!(matching_paper((w.round(), h.round())), Some(i));
        assert_eq!(matching_paper((w + 0.51, *h)), None);
    }
    for size in [(0.0, 0.0), (-1728.0, 2592.0), (f64::INFINITY, 2592.0), (1728.0, f64::NAN), (1000.0, 1000.0)] {
        assert_eq!(matching_paper(size), None);
    }
}

#[test]
fn label_is_cut_turned_upright_and_fitted_without_distortion() {
    let label = LABEL_PAPER;
    assert_eq!(PAPERS[label].0, "4 x 6 in label (101.6 x 152.4 mm)");
    assert_eq!(PAPERS[label].1, (288.0, 432.0));
    assert_eq!(cups_media(label), Some("w288h432"));
    // A sideways label in the lower half of an A4 sheet, text running bottom-to-top.
    let rect = [40.0, 30.0, 560.0, 420.0];
    for (text_up, flip, want_text_dir) in [(true, false, (1.0, 0.0)), (false, false, (1.0, 0.0)), (true, true, (-1.0, 0.0))] {
        // Text direction on the page: up when text_up, down otherwise.
        let page_dir = if text_up { (0.0, 1.0) } else { (0.0, -1.0) };
        let s = Settings {
            pages: vec![0],
            paper: PAPERS[label].1,
            layout: Layout::Label { flip },
            crops: vec![Some(Crop { rect, text_up })],
            ..Settings::default()
        };
        let sheets = layout(&[(595.0, 842.0)], &s).unwrap();
        assert_eq!(sheets.len(), 1);
        assert_eq!(sheets[0].size, (288.0, 432.0));
        let pl = sheets[0].placed[0];
        assert_eq!(pl.clip, rect);
        let [a, b, c, d, _, _] = pl.matrix.0;
        // A pure quarter turn with one scale: no stretching.
        assert!((a.hypot(b) - c.hypot(d)).abs() < 1e-9 && (a * c + b * d).abs() < 1e-9);
        let dir = (a * page_dir.0 + c * page_dir.1, b * page_dir.0 + d * page_dir.1);
        let n = dir.0.hypot(dir.1);
        assert!((dir.0 / n - want_text_dir.0).abs() < 1e-9 && (dir.1 / n - want_text_dir.1).abs() < 1e-9, "text_up={text_up} flip={flip}");
        let corners = [(rect[0], rect[1]), (rect[2], rect[1]), (rect[2], rect[3]), (rect[0], rect[3])].map(|(x, y)| pl.matrix.apply(x, y));
        let (x0, x1) = corners.iter().fold((f64::MAX, f64::MIN), |m, p| (m.0.min(p.0), m.1.max(p.0)));
        let (y0, y1) = corners.iter().fold((f64::MAX, f64::MIN), |m, p| (m.0.min(p.1), m.1.max(p.1)));
        assert!(x0 >= LABEL_MARGIN - 1e-6 && x1 <= 288.0 - LABEL_MARGIN + 1e-6 && y0 >= LABEL_MARGIN - 1e-6 && y1 <= 432.0 - LABEL_MARGIN + 1e-6);
        // Fills the label's width (the crop's height, 390 pt, becomes it).
        assert!((x1 - x0 - (288.0 - 2.0 * LABEL_MARGIN)).abs() < 1e-6);
    }
    // No crop: the whole page, upright.
    let whole =
        layout(&[(200.0, 300.0)], &Settings { pages: vec![0], paper: (288.0, 432.0), layout: Layout::Label { flip: false }, ..Settings::default() })
            .unwrap();
    assert_eq!(whole[0].placed[0].clip, [0.0, 0.0, 200.0, 300.0]);
    assert_eq!(whole[0].placed[0].matrix.0[1], 0.0);
}

#[test]
fn text_direction_is_read_from_text_and_form_matrices() {
    let doc = fixture(1);
    assert!(!text_reads_up(&doc, 0), "upright text");
    assert!(!text_reads_up(&doc, 9), "no such page");
}
