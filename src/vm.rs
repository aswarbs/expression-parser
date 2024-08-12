use crate::tokens::TAMInst;

fn execute(stack: &mut Vec<i32>, instruction: TAMInst) -> Result<&mut Vec<i32>, String> {
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
    }

    Ok(stack)
}

pub fn exec_tam(stack: &mut Vec<i32>, instructions: Vec<TAMInst>) -> Result<&mut Vec<i32>, String> {
    instructions
        .into_iter()
        .try_fold(stack, |s, instr| execute(s,instr))
}