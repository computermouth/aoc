
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ARRAY_LENGTH(a) (sizeof((a))/sizeof((a)[0]))

char test0[] = 
"5	1	9	5\n"
"7	5	3\n"
"2	4	6	8\n";

char input[] =
"493	458	321	120	49	432	433	92	54	452	41	461	388	409	263	58\n"
"961	98	518	188	958	114	1044	881	948	590	972	398	115	116	451	492\n"
"76	783	709	489	617	72	824	452	748	737	691	90	94	77	84	756\n"
"204	217	90	335	220	127	302	205	242	202	259	110	118	111	200	112\n"
"249	679	4015	106	3358	1642	228	4559	307	193	4407	3984	3546	2635	3858	924\n"
"1151	1060	2002	168	3635	3515	3158	141	4009	3725	996	142	3672	153	134	1438\n"
"95	600	1171	1896	174	1852	1616	928	79	1308	2016	88	80	1559	1183	107\n"
"187	567	432	553	69	38	131	166	93	132	498	153	441	451	172	575\n"
"216	599	480	208	224	240	349	593	516	450	385	188	482	461	635	220\n"
"788	1263	1119	1391	1464	179	1200	621	1304	55	700	1275	226	57	43	51\n"
"1571	58	1331	1253	60	1496	1261	1298	1500	1303	201	73	1023	582	69	339\n"
"80	438	467	512	381	74	259	73	88	448	386	509	346	61	447	435\n"
"215	679	117	645	137	426	195	619	268	223	792	200	720	260	303	603\n"
"631	481	185	135	665	641	492	408	164	132	478	188	444	378	633	516\n"
"1165	1119	194	280	223	1181	267	898	1108	124	618	1135	817	997	129	227\n"
"404	1757	358	2293	2626	87	613	95	1658	147	75	930	2394	2349	86	385\n";

typedef struct {
	short **table;
	size_t rows;
	size_t *columns;
} payload;

int process(payload p){
	
	int checksum = 0;
	
	for(int i = 0; i < p.rows; i++){
		
		int max = p.table[i][0];
		int min = p.table[i][0];
		
		for(int j = 0; j < p.columns[i]; j++){
			if (p.table[i][j] > max) {
				max = p.table[i][j];
			} else if (p.table[i][j] < min) {
				min = p.table[i][j];
			}
		}
		
		checksum += max - min;
	}
	
	return checksum;
}

payload init_p(char * input_string, size_t len){
	
	payload new_p = { .table = NULL, .rows = 0, .columns = NULL };
	
	char split_n[] = "\n";
	char *save_n = NULL;
	char *token_n = NULL;
	
	token_n = strtok_r(input_string, split_n, &save_n);
	while( token_n != NULL ){
		
		new_p.rows += 1;
		
		new_p.table = realloc(new_p.table, sizeof(short *) * new_p.rows);
		new_p.table[new_p.rows - 1] = NULL;
		
		char split_t[] = "\t";
		char *save_t = NULL;
		char *token_t = NULL;
		
		size_t tmp_col = 0;
		
		char *substring;
		substring = strdup(token_n);
		
		token_t = strtok_r(substring, split_t, &save_t);
		while( token_t != NULL ){
			
			tmp_col += 1;
			
			new_p.table[new_p.rows - 1] = realloc(new_p.table[new_p.rows - 1], sizeof(short) * tmp_col);
			new_p.table[new_p.rows - 1][tmp_col - 1] = atoi(token_t);
			
			token_t = strtok_r(NULL, split_t, &save_t);
		}
		
		free(substring);
		
		new_p.columns = realloc(new_p.columns, sizeof(size_t) * new_p.rows);
		new_p.columns[new_p.rows - 1] = tmp_col;
		
		token_n = strtok_r(NULL, split_n, &save_n);
	}
	
	
	printf("len: %lu\n", len);
	printf("rows: %lu\n", new_p.rows);
	printf("columns:");
	for(int i = 0; i < new_p.rows; i++)
		printf(" %lu", new_p.columns[i]);
	printf("\n");
	
	return new_p;
}

void destroy_p(payload p){
	
	for(int i = 0; i < p.rows; i++){
		free(p.table[i]);
	}
	free(p.table);
	
	free(p.columns);
}

int main(){
	
	payload p0 = init_p(test0, strlen(test0));
	printf("chk: %d\n\n", process(p0));
	destroy_p(p0);
	
	payload p1 = init_p(input, strlen(input));
	printf("chk: %d\n\n", process(p1));
	destroy_p(p1);
	
	return 0;
}
