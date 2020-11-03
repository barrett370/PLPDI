#include <stdio.h>
#include <malloc.h>

#define MAXARG 3 

#define INT 0
#define ADD 1
#define MUL 2
#define LET 3
#define VAR 4


typedef struct expr
{
  char tag;
  void *data; //box the data
  struct expr *operands[MAXARG];
} expr_t;

typedef struct var {
    char name[1];
    void *data;
} var_t ;


expr_t* eval(expr_t *e)
{
    expr_t *e0, *e1, *e2, *et;
    int *d0, *d1, *d2, *v0;
    char *n0;
    switch(e->tag)
    {
        case VAR:
             
            return NULL;
        case INT:
            return e;
        case ADD:
            et = e->operands[0];
            e0 = eval (et);
            d0 = e0 -> data;
            et = e->operands[1];
            e1 = eval (et);
            d1 = e1 -> data;
            e2  = (expr_t *) malloc (sizeof (expr_t));
            e2->tag = INT;
            d2 = malloc (sizeof (int));
            (*d2) = *d0 + *d1;
            e2->data = d2;
            return e2;
        case MUL:
            et = e->operands[0];
            e0 = eval (et);
            d0 = e0 -> data;
            et = e->operands[1];
            e1 = eval (et);
            d1 = e1 -> data;
            e2 = (expr_t *) malloc (sizeof (expr_t));
            d2 = malloc (sizeof (int));
            (*d2) = *d0 * *d1;
            e2 -> data = d2;
            return e2;
        //case LET:
        //    // value assigned becomes operands where value is referenced
        //    n0 = eval( e->operands[0]) -> data; //var name
        //    v0 = eval (e->operands[1]) -> data; //var val
        //    var_t *v1;
        //    v1 = (var_t *) malloc(sizeof(var_t));
        //    strcpy(v1->name ,n0);
        //    v1->data = v0; 
        //    expr_t *body = e->operands[2];
        //    body->vars[0] = v1;
        //    e1 = eval(body);
        //    return e1;
        default:
            return NULL;
    }
}

int
main ()
{
  int *data;
  expr_t *c1, *c2, *c3, *c4, *s1, *s2, *s3,*s4, *e,*e2;
  
  // This should be created by the parser
  
  // Constant: 1
  c1 = (expr_t *) malloc (sizeof (expr_t));
  c1->tag = INT;
  data = malloc (sizeof (int));
  (*data) = 1;
  c1->data = data;

  // Constant: 2
  c2 = (expr_t *) malloc (sizeof (expr_t));
  c2->tag = INT;
  data = malloc (sizeof (int));
  (*data) = 2;
  c2->data = data;

  // Sum: 1 + 2
  s1 = (expr_t *) malloc (sizeof(expr_t));
  s1->tag = ADD;
  s1->data = NULL;
  s1->operands[0] = c1;
  s1->operands[1] = c2;
  
  // Constant: 1
  c3 = (expr_t *) malloc (sizeof (expr_t));
  c3->tag = INT;
  data = malloc (sizeof (int));
  (*data) = 1;
  c3->data = data;

  // Constant: 2
  c4 = (expr_t *) malloc (sizeof (expr_t));
  c4->tag = INT;
  data = malloc (sizeof (int));
  (*data) = 2;
  c4->data = data;

  // Sum: 1 + 2
  s2 = (expr_t *) malloc (sizeof(expr_t));
  s2->tag = ADD;
  s2->data = NULL;
  s2->operands[0] = c3;
  s2->operands[1] = c4;
  
  // Sum: (1+2)+(1+2)
  s3 = (expr_t *) malloc (sizeof(expr_t));
  s3->tag = ADD;
  s3->data = NULL;
  s3->operands[0] = s1;
  s3->operands[1] = s2;


  // Mul (2 * 2) 
  
  s4 = (expr_t *) malloc (sizeof(expr_t));
  s4->tag = MUL;
  s4->data = NULL;
  s4->operands[0] = c4;
  s4->operands[1] = c4;



  // Task 0: Add code for MUL  DONE
  // Task 1: Attempt to extend the expression language 
  //         with a ternary LET construct DONE
  // Task 2: Improve the evaluator to deal with LET
  // Task 3: Construct the ASG for the expression 
  //         let x = 1 + 2 in x + x instead
  // Task 4: Evaluate the ASG
  // Task 5: Re-implement without using language recursion
  // Task 6: Optimise memory utilisation. 

  // run the evaluator
  e = eval(s3);    
  e2 = eval(s4);
  printf("%d",*(int *)(e2->data));
  return *((int*)(e->data));
}
