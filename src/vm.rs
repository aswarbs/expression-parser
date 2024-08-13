use crate::tokens::TAMInst;

fn execute(stack: &mut Vec<i32>, instruction: TAMInst, trace:bool) -> Result<&mut Vec<i32>, String> {
    match instruction {
        TAMInst::LOADL(value) => { stack.push(value); }
        TAMInst::ADD => {
            let x = stack.pop().ok_or("Missing stack value")?;
            let y = stack.pop().ok_or("Missing stack value")?;
            stack.push(y + x);
        }
        TAMInst::SUB => {
            let x = stack.pop().ok_or("Missing stack value")?;
            let y = stack.pop().ok_or("Missing stack value")?;
            stack.push(y - x);
        }
        TAMInst::MUL => {
            let x = stack.pop().ok_or("Missing stack value")?;
            let y = stack.pop().ok_or("Missing stack value")?;
            stack.push(y * x);
        }
        TAMInst::DIV => {
            let x = stack.pop().ok_or("Missing stack value")?;
            let y = stack.pop().ok_or("Missing stack value")?;
            stack.push(y / x);
        }
        TAMInst::MOD => {
            let x = stack.pop().ok_or("Missing stack value")?;
            let y = stack.pop().ok_or("Missing stack value")?;
            stack.push(y%x);
        }
        TAMInst::NEG => {
            let x = stack.pop().ok_or("Missing stack value")?;
            stack.push(-x);
        }
        TAMInst::ABS => {
            let x = stack.pop().ok_or("Missing stack value")?;
            stack.push(x.abs());
        }
        TAMInst::AND => {
            let x = stack.pop().ok_or("Missing stack value")?;
            let y = stack.pop().ok_or("Missing stack value")?;
            if x>= 1 && y >= 1 {stack.push(1)} else {stack.push(0)};
        }
        TAMInst::OR => {
            let x = stack.pop().ok_or("Missing stack value")?;
            let y = stack.pop().ok_or("Missing stack value")?;
            if x>= 1 || y >= 1 {stack.push(1)} else {stack.push(0)};
        }
        TAMInst::NOT => {
            let x = stack.pop().ok_or("Missing stack value")?;
            if x==0 {stack.push(1)} else {stack.push(0)}
        }
        TAMInst::GTR => {
            let x = stack.pop().ok_or("Missing stack value")?;
            let y = stack.pop().ok_or("Missing stack value")?;
            stack.push(i32::from(y > x));
        }
        TAMInst::LSS => {
            let x = stack.pop().ok_or("Missing stack value")?;
            let y = stack.pop().ok_or("Missing stack value")?;
            stack.push(i32::from(y < x));
        }
        TAMInst::EQL => {
            let x = stack.pop().ok_or("Missing stack value")?;
            let y = stack.pop().ok_or("Missing stack value")?;
            stack.push(i32::from(y == x));}
    }

    if trace {println!("{:?}          {:?}", instruction, stack);}

    Ok(stack)
}

pub fn exec_tam(stack: &mut Vec<i32>, instructions: Vec<TAMInst>, trace:bool) -> Result<&mut Vec<i32>, String> {
    if trace {println!("instruction || stack");}
    instructions
        .into_iter()
        .try_fold(stack, |s, instr| { execute(s,instr,trace) })
}