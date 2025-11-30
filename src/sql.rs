use crate::payment_type::{self, Payment};
use rusqlite::{Connection, Result};

pub fn create(conn: &mut Connection) -> Result<()> {
    conn.execute(
        "create table if not exists payments (
             id integer primary key AUTOINCREMENT,
             name TEXT,
             amount INTGERR
         )",
        (),
    )?;
    Ok(())
}

pub fn insert_payment(conn: &mut Connection, payment: &payment_type::Payment) -> Result<()> {
    conn.execute(
        "INSERT INTO payments (name, amount) VALUES (?1, ?2)",
        (&payment.name, &payment.amount),
    )?;
    Ok(())
}

pub fn list_payment(conn: &mut Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, amount FROM payments")?;
    let payments_iter = stmt.query_map([], |row| {
        Ok(payment_type::Payment {
            id: row.get(0)?,
            name: row.get(1)?,
            amount: row.get(2)?,
        })
    })?;
    println!("| ID | Name | Amount |",);
    for payment in payments_iter {
        let paymetn_row = payment.unwrap().clone();
        println!(
            "| {:?} | {:?} | {:?} |",
            paymetn_row.id.unwrap(),
            paymetn_row.name,
            paymetn_row.amount
        );
    }
    Ok(())
}

pub fn get_payments(conn: &mut Connection) -> Result<Vec<Payment>> {
    let mut stmt = conn.prepare("SELECT id, name, amount FROM payments")?;
    let mut payments: Vec<Payment> = vec![];
    let payments_iter = stmt.query_map([], |row| {
        Ok(payment_type::Payment {
            id: row.get(0)?,
            name: row.get(1)?,
            amount: row.get(2)?,
        })
    })?;
    for payment in payments_iter {
        let payment_row = payment.unwrap().clone();
        let item = Payment {
            id: payment_row.id,
            name: payment_row.name,
            amount: payment_row.amount,
        };
        payments.push(item);
    }
    Ok(payments)
}

pub fn remove_payment(conn: &mut Connection, id: &u32) -> Result<()> {
    let mut stmt = conn.prepare("DELETE FROM payments WHERE id=(?1)")?;
    stmt.execute([id])?;
    Ok(())
}
