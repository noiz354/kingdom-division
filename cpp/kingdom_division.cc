/* *******************************
    -----------------------------
   | copyrights with l0gic_b0mb  |
    -----------------------------
   ******************************* */
 
//#include "/Users/normansyahputa/stdc++.h"
#include<bits/stdc++.h>
#define MOD 1000000007
#define llint long long int
#define max(a,b) (a>=b?a:b)
#define min(a,b) (a<=b?a:b)
using namespace std;
 
vector<int> graph[100001];
llint dp[100001][2][3],N,u,v,parent[100001];
bool visited[100001];
 
bool leaf(int node)
{
    bool l=true;
    for(int i=0;i<graph[node].size();i++)
    {
        if(graph[node][i]!=parent[node])
        {
            l=false;
        }
    }
    return l;
}
 
void DFS(int source)
{
    visited[source]=true;
    for(int i=0;i<graph[source].size();i++)
    {
        if(!visited[graph[source][i]])
        {
            parent[graph[source][i]]=source;
            DFS(graph[source][i]);
        }
    }
    return;
}
 
llint solve_for(int root,int color,int streak)
{
    if(dp[root][color][streak]!=-1)
        return dp[root][color][streak];
    if(leaf(root))
    {
        if(streak==1)
            return dp[root][color][streak]=0;
        if(streak==2)
        return dp[root][color][streak]=1;
    }
    llint ans=1,valid,invalid=1;
    for(int i=0;i<graph[root].size();i++)
    {
        int node=graph[root][i];
        if(node!=parent[root])
        {
            valid=0;
            valid+=solve_for(node,!color,1);
            invalid=(invalid*solve_for(node,!color,1))%MOD;
            valid+=solve_for(node,color,2);
            ans=(ans*valid)%MOD;
        }
    }
cout<<root<<" "<<color<<" "<<streak<<" "<<valid<<" "<<invalid<<endl;
    if(streak==1)
    {
        ans=(ans-invalid+MOD)%MOD;
    }
    return dp[root][color][streak]=ans;
}
 
void reset()
{
    for(int i=1;i<=N;i++)
    {
        for(int j=1;j<=2;j++)
        {
            dp[i][0][j]=dp[i][1][j]=-1;
        }
    }
    return;
}
 
int main()
{
   ios::sync_with_stdio(0);
   cin.tie(0); cout.tie(0);
   cin>>N;
   for(int i=1;i<N;i++)
   {
       cin>>u>>v;
       graph[u].push_back(v);
       graph[v].push_back(u);
   }
   reset();
   DFS(1);
   llint ans=(solve_for(1,0,1)*2)%MOD;
   cout<<ans<<endl;
   return 0;
}
 
/* Hidden within the kernel is a l0gic_b0mb; malicious code designed to execute under circumstances I've programmed.*/
