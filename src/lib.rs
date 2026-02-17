#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElevatorError {
    InvalidFloor (i32 ),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue,
}

#[derive(Debug)]
pub struct Elevator {
    pub floor: i32,
    pub state: State,
    pub queue: Vec<i32>,
}

impl Elevator {
    pub fn new(floor: i32) -> Result<Self, ElevatorError> {
        if (floor < 0) || (floor > 5) {
            return Err(ElevatorError::InvalidFloor (floor));
        }
        Ok(Self {
            floor,
            state: State::Idle,
            queue: Vec::new(),
        })
    }

    

    pub fn floor(&self) -> i32 {
        self.floor
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn queue(&self) -> &[i32] {
        &self.queue
    }

    pub fn call(&mut self, floor: i32) -> Result<(), ElevatorError> {
        if (floor < 0) || (floor > 5) {
            return Err(ElevatorError::InvalidFloor(floor));
        }
      
    
        if floor == self.floor {
            return Ok(());
        }

        
        if self.queue.contains(&floor) { //safe tu use even when vec is empty 
            return Ok(());
        }

        self.queue.push(floor);

        if self.state == State::Idle {
            let target = self.queue[0];
            if target > self.floor {
                self.state = State::MovingUp;
            } else {
                self.state = State::MovingDown;
            }
        }

        Ok(())
    }
    pub fn step(&mut self) -> Result<(), ElevatorError> {
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::CannotMoveDoorsOpen);
        }
        if self.queue.is_empty() {
            return Err(ElevatorError::EmptyQueue)
        }

        if self.queue.is_empty() {
            return Err(ElevatorError::EmptyQueue);
        }

        let target = self.queue[0];

        if self.floor > target {
            self.floor -= 1;
        } else if self.floor < target {
            self.floor += 1;
        }

        if self.floor == target {
            self.state = State::DoorsOpen;
            self.queue.remove(0);
        } else if self.floor > target {
            self.state = State::MovingDown;
        } else {
            self.state = State::MovingUp;
        }

        Ok(())
    }
    pub fn open_doors(&mut self) -> Result<(), ElevatorError> {
    
        if self.state ==State::DoorsOpen {
           return Err(ElevatorError::CannotMoveDoorsOpen);
        }

        if self.state != State::Idle {
           return Err(ElevatorError::CannotOpenWhileMoving);
        }

        self.state= State::DoorsOpen ;
        Ok(())
    }

    pub fn close_doors(&mut self) -> Result<(), ElevatorError> {
        if self.state!=State::DoorsOpen{
            return Err(ElevatorError::DoorsAlreadyClosed);
        }

        if !self.queue.is_empty(){
            let target = self.queue[0] ;
            if target>self.floor {
                self.state = State::MovingUp
            } else {
                self.state =State::MovingDown
            }
        }
        Ok(())
    
    }
    //equi constructeur copie 
    pub fn status(&self) -> Self {
        Self {
            floor: self.floor(),
            state: self.state() ,
            queue: self.queue().to_vec(), // clone por les objest compose 
        }


    }
    

} 


fn main() {
    
}
