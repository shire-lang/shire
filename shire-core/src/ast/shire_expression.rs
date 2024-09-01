// use std::collections::HashMap;
// use crate::ast::front_matter_type::FrontMatterType;
// use crate::ast::pattern_action_fun::PatternActionFunc;
//
// // 定义枚举，类似于 sealed class
// enum StatementType {
//     Operator(Operator),
//     StringOperator(StringOperatorStatement),
//     Comparison(Comparison),
//     StringComparison(StringComparison),
//     LogicalExpression(LogicalExpression),
//     NotExpression(NotExpression),
//     MethodCall(MethodCall),
//     Value(Value),
//     Processor(Processor),
//     CaseKeyValue(CaseKeyValue),
//     ConditionCase(ConditionCase),
// }
//
// trait Statement {
//     // evaluate 函数
//     fn evaluate(&self, variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         match &self {
//             StatementType::Operator(op) => Ok(Box::new(op.type_.display().clone())),
//             StatementType::StringOperator(op) => Ok(Box::new(op.type_.display().clone())),
//             StatementType::Comparison(comp) => Ok(Box::new(comp.evaluate(variables))),
//             StatementType::StringComparison(comp) => Ok(Box::new(comp.evaluate(variables))),
//             StatementType::LogicalExpression(expr) => Ok(Box::new(expr.evaluate(variables))),
//             StatementType::NotExpression(expr) => Ok(Box::new(expr.evaluate(variables))),
//             StatementType::MethodCall(call) => Ok(Box::new(call.evaluate(variables))),
//             StatementType::Value(val) => Ok(Box::new(val.evaluate(variables))),
//             StatementType::Processor(proc) => Ok(Box::new(proc.evaluate(variables))),
//             StatementType::CaseKeyValue(case) => Ok(Box::new(case.evaluate(variables))),
//             StatementType::ConditionCase(cond) => Ok(Box::new(cond.evaluate(variables))),
//         }
//     }
//
//     fn display(&self) -> String {
//         match self {
//             StatementType::Operator(op) => format!("{}", op.type_.display()),
//             StatementType::StringOperator(op) => format!("{}", op.type_.display()),
//             StatementType::Comparison(comp) => format!(
//                 "{} {} {}",
//                 comp.left.display(),
//                 comp.operator.type_.display(),
//                 comp.right.display()
//             ),
//             StatementType::StringComparison(comp) => format!(
//                 "{} {} {}",
//                 comp.variable,
//                 comp.operator.type_.display(),
//                 comp.value
//             ),
//             StatementType::LogicalExpression(expr) => format!(
//                 "{} {} {}",
//                 expr.left.as_ref().display(),
//                 expr.operator.display(),
//                 expr.right.as_ref().display()
//             ),
//             StatementType::NotExpression(expr) => format!("!{}", expr.operand.as_ref().display()),
//             StatementType::MethodCall(call) => {
//                 let parameters = call.arguments.as_ref().map(|args| {
//                     args.iter()
//                         .map(|arg| match arg {
//                             FrontMatterType::STRING(s) => s.clone(),
//                             _ => format!("{}", arg),
//                         })
//                         .collect::<Vec<_>>()
//                         .join(", ")
//                 }).unwrap_or_default();
//
//                 let formatted_parameters = if parameters.is_empty() {
//                     "".to_string()
//                 } else {
//                     format!("({})", parameters)
//                 };
//
//                 let dot_with_target = if call.method_name == FrontMatterType::EMPTY {
//                     "".to_string()
//                 } else if let FrontMatterType::IDENTIFIER(name) = &call.method_name {
//                     if name.is_empty() {
//                         "".to_string()
//                     } else {
//                         format!(".{}", call.method_name.display())
//                     }
//                 } else {
//                     format!(".{}", call.method_name.display())
//                 };
//
//                 format!(
//                     "{}{}{}",
//                     call.object_name.display(),
//                     dot_with_target,
//                     formatted_parameters
//                 )
//             }
//             StatementType::Value(val) => val.value.display(),
//             StatementType::Processor(proc) => proc.processors.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" | "),
//             _ => "Unsupported statement type".to_string(),
//         }
//     }
// }
//
// // 实现 Value 结构体
// struct Value {
//     value: FrontMatterType,
// }
//
// impl Statement for Value {
//     fn evaluate(&self, _variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         let result: Box<dyn std::any::Any> = match &self.value {
//             FrontMatterType::STRING(val) => Box::new(val.clone()),
//             FrontMatterType::NUMBER(val) => Box::new(*val),
//             FrontMatterType::DATE(val) => Box::new(val.clone()),
//             FrontMatterType::BOOLEAN(val) => Box::new(*val),
//             _ => return Err(format!("Unsupported value type: {:?}", self.value)),
//         };
//         Ok(result)
//     }
// }
//
// // 定义 OperatorType 枚举
// enum OperatorType {
//     Or,
//     And,
//     Not,
//     Equal,
//     NotEqual,
//     LessThan,
//     GreaterThan,
//     LessEqual,
//     GreaterEqual,
// }
//
// impl OperatorType {
//     fn from_str(operator: &str) -> Result<Self, String> {
//         match operator {
//             "||" => Ok(OperatorType::Or),
//             "&&" => Ok(OperatorType::And),
//             "!" => Ok(OperatorType::Not),
//             "==" => Ok(OperatorType::Equal),
//             "!=" => Ok(OperatorType::NotEqual),
//             "<" => Ok(OperatorType::LessThan),
//             ">" => Ok(OperatorType::GreaterThan),
//             "<=" => Ok(OperatorType::LessEqual),
//             ">=" => Ok(OperatorType::GreaterEqual),
//             _ => Err(format!("Invalid operator: {}", operator)),
//         }
//     }
// }
//
// impl Statement for OperatorType {
//     fn display(&self) -> String {
//         match self {
//             OperatorType::Or => format!("{}", "||"),
//             OperatorType::And => format!("{}", "&&"),
//             OperatorType::Not => format!("{}", "!"),
//             OperatorType::Equal => format!("{}", "=="),
//             OperatorType::NotEqual => format!("{}", "!="),
//             OperatorType::LessThan => format!("{}", "<"),
//             OperatorType::GreaterThan => format!("{}", ">"),
//             OperatorType::LessEqual => format!("{}", "<="),
//             OperatorType::GreaterEqual => format!("{}", ">="),
//         }
//     }
//
// }
//
// // 实现 StringOperator 枚举
// enum StringOperator {
//     Contains,
//     StartsWith,
//     EndsWith,
//     Matches,
// }
//
// impl Statement for StringOperator {
//     fn display(&self) -> String {
//         match self {
//             StringOperator::Contains => format!("{}", "contains"),
//             StringOperator::StartsWith => format!("{}", "startsWith"),
//             StringOperator::EndsWith => format!("{}", "endsWith"),
//             StringOperator::Matches => format!("{}", "matches"),
//         }
//     }
// }
//
// // Operator 结构体
// struct Operator {
//     type_: OperatorType,
// }
//
// impl Statement for Operator {
//     // fn evaluate(&self, _variables: &HashMap<String, String>) -> Box<dyn std::any::Any> {
//     //     Box::new(self.type_.display().to_string())
//     // }
//     fn evaluate(&self, _variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         Ok(Box::new(self.type_.display().to_string()))
//     }
// }
//
// // StringOperatorStatement 结构体
// struct StringOperatorStatement {
//     type_: StringOperator,
// }
//
// impl Statement for StringOperatorStatement {
//     // fn evaluate(&self, _variables: &HashMap<String, String>) -> Box<dyn std::any::Any> {
//     //     Box::new(self.type_.display().to_string())
//     // }
//     fn evaluate(&self, _variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         Ok(Box::new(self.type_.display().to_string()))
//     }
// }
//
// // Comparison 结构体
// struct Comparison {
//     left: FrontMatterType,
//     operator: Operator,
//     right: FrontMatterType,
// }
//
// impl Statement for Comparison {
//     fn evaluate(&self, variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         let left_value = match &self.left {
//             FrontMatterType::STRING(val) => val.clone(),
//             FrontMatterType::VARIABLE(var) => variables.get(var).cloned().unwrap_or_else(|| "".to_string()),
//             _ => return Err("Unsupported left value type".to_string()),
//         };
//
//         let right_value = match &self.right {
//             FrontMatterType::STRING(val) => val.clone(),
//             _ => return Err("Unsupported right value type".to_string()),
//         };
//
//         let result = match self.operator.type_ {
//             OperatorType::Equal => left_value == right_value,
//             OperatorType::NotEqual => left_value != right_value,
//             OperatorType::LessThan => left_value < right_value,
//             OperatorType::GreaterThan => left_value > right_value,
//             OperatorType::LessEqual => left_value <= right_value,
//             OperatorType::GreaterEqual => left_value >= right_value,
//             _ => return Err("Invalid comparison operator".to_string()),
//         };
//
//         Ok(Box::new(result))
//     }
// }
//
// // StringComparison 结构体
// struct StringComparison {
//     variable: String,
//     operator: StringOperatorStatement,
//     value: String,
// }
//
// impl Statement for StringComparison {
//     fn evaluate(&self, _variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         let result = match self.operator.type_ {
//             StringOperator::Contains => self.variable.contains(&self.value),
//             StringOperator::StartsWith => self.variable.starts_with(&self.value),
//             StringOperator::EndsWith => self.variable.ends_with(&self.value),
//             StringOperator::Matches => {
//                 match regex::Regex::new(&self.value) {
//                     Ok(regex) => regex.is_match(&self.variable),
//                     Err(_) => return Err("Invalid regex pattern".to_string()),
//                 }
//             }
//         };
//
//         Ok(Box::new(result))
//     }
// }
//
// // LogicalExpression 结构体
// struct LogicalExpression {
//     left: Box<StatementType>,
//     operator: OperatorType,
//     right: Box<StatementType>,
// }
//
// impl Statement for LogicalExpression {
//     fn evaluate(&self, variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         // Evaluate the left and right operands
//         let left_result = self.left.as_ref().evaluate(variables);
//         let right_result = self.right.as_ref().evaluate(variables);
//
//         // Downcast the results to booleans
//         let left_value = match left_result.downcast_ref::<bool>() {
//             Some(value) => value,
//             None => return Err("Left operand is not of type bool".to_string()),
//         };
//
//         let right_value = match right_result.downcast_ref::<bool>() {
//             Some(value) => value,
//             None => return Err("Right operand is not of type bool".to_string()),
//         };
//
//         // Compute the result based on the operator
//         let result = match self.operator {
//             OperatorType::And => *left_value && *right_value,
//             OperatorType::Or => *left_value || *right_value,
//             _ => return Err("Invalid logical operator".to_string()),
//         };
//
//         // Return the result as a Box<dyn Any> wrapped in Ok
//         Ok(Box::new(result))
//     }
// }
//
// // NotExpression 结构体
// struct NotExpression {
//     operand: Box<StatementType>,
// }
//
// impl Statement for NotExpression {
//     fn evaluate(&self, variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         // Evaluate the operand and get the result as a Box<dyn Any>
//         let operand_result = self.operand.as_ref().evaluate(variables);
//
//         // Attempt to downcast the result to a boolean
//         let operand_value = match operand_result.downcast_ref::<bool>() {
//             Some(value) => value,
//             None => return Err("Operand is not of type bool".to_string()),
//         };
//
//         // Compute the negation of the boolean value
//         let result = !*operand_value;
//
//         // Return the result as a Box<dyn Any> wrapped in Ok
//         Ok(Box::new(result))
//     }
// }
//
// // MethodCall 结构体
// struct MethodCall {
//     object_name: FrontMatterType,
//     method_name: FrontMatterType,
//     arguments: Option<Vec<FrontMatterType>>,
// }
//
// impl MethodCall {
//     fn parameters(&self) -> Option<Vec<String>> {
//         self.arguments.as_ref().map(|args| {
//             args.iter()
//                 .map(|arg| match arg {
//                     FrontMatterType::STRING(s) => s.clone(),
//                     _ => arg.display(),
//                 })
//                 .collect()
//         })
//     }
//
//     fn evaluate_expression(
//         &self,
//         method_name: &str,
//         parameters: Option<Vec<String>>,
//         value: &str,
//     ) -> Box<dyn std::any::Any> {
//         match method_name {
//             "length" => Box::new(value.len()),
//             "trim" => Box::new(value.trim().to_string()),
//             "contains" => {
//                 let param = parameters.unwrap().get(0).unwrap();
//                 Box::new(value.contains(param))
//             }
//             "startsWith" => {
//                 let param = parameters.unwrap().get(0).unwrap();
//                 Box::new(value.starts_with(param))
//             }
//             "endsWith" => {
//                 let param = parameters.unwrap().get(0).unwrap();
//                 Box::new(value.ends_with(param))
//             }
//             "lowercase" => Box::new(value.to_lowercase()),
//             "uppercase" => Box::new(value.to_uppercase()),
//             "isEmpty" => Box::new(value.is_empty()),
//             "isNotEmpty" => Box::new(!value.is_empty()),
//             "first" => Box::new(value.chars().next().unwrap().to_string()),
//             "last" => Box::new(value.chars().last().unwrap().to_string()),
//             "matches" => {
//                 let param = parameters.unwrap().get(0).unwrap();
//                 let regex = regex::Regex::new(param).unwrap();
//                 Box::new(regex.is_match(value))
//             }
//             _ => panic!("Unsupported method: {}", method_name),
//         }
//     }
// }
//
// impl Statement for MethodCall {
//     fn evaluate(&self, variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         // Resolve the object name to a string value
//         let value = match &self.object_name {
//             FrontMatterType::STRING(s) => s.clone(),
//             FrontMatterType::VARIABLE(var) => variables.get(var).cloned().unwrap_or_else(|| "".to_string()),
//             _ => return Err("Unsupported object name type".to_string()),
//         };
//
//         // Prepare method name and parameters
//         let method_name = self.method_name.display();
//         let parameters = self.parameters();
//
//         // Evaluate the expression and handle potential errors
//         self.evaluate_expression(&method_name, parameters, &value)
//             .map(|result| Box::new(result) as Box<dyn std::any::Any>)
//             .map_err(|e| e.to_string())
//     }
// }
//
// // Processor 结构体
// struct Processor {
//     processors: Vec<PatternActionFunc>,
// }
//
// impl Statement for Processor {
//     // fn evaluate(&self, _variables: &HashMap<String, String>) -> Vec<PatternActionFunc> {
//     //     self.processors.clone()
//     // }
//     fn evaluate(&self, _variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         // Clone the processors vector
//         let processors = self.processors.clone();
//
//         // Convert Vec<PatternActionFunc> to Box<dyn std::any::Any>
//         Ok(Box::new(processors) as Box<dyn std::any::Any>)
//     }
// }
//
// // CaseKeyValue 结构体
// struct CaseKeyValue {
//     key: FrontMatterType,
//     value: FrontMatterType,
// }
//
// impl Statement for CaseKeyValue {
//     fn evaluate(&self, _variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         // Create the tuple from the key and value
//         let result = (
//             self.key.display(),
//             self.value.display()
//         );
//
//         // Return the tuple boxed as Box<dyn Any>
//         Ok(Box::new(result) as Box<dyn std::any::Any>)
//     }
// }
//
// // ConditionCase 结构体
// struct ConditionCase {
//     conditions: Vec<FrontMatterType>,
//     cases: Vec<FrontMatterType>,
// }
//
// impl Statement for ConditionCase {
//     fn evaluate(&self, _variables: &HashMap<String, String>) -> Result<Box<dyn std::any::Any>, String> {
//         // Create vectors of strings from the conditions and cases
//         let condition: Vec<String> = self.conditions.iter().map(|cond| cond.display()).collect();
//         let case: Vec<String> = self.cases.iter().map(|case| case.display()).collect();
//
//         // Create the tuple of vectors
//         let result = (condition, case);
//
//         // Box the tuple and return it
//         Ok(Box::new(result) as Box<dyn std::any::Any>)
//     }
// }
