library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity queue is
    port (
        clk, rst : in std_logic;
        push : in std_logic;
        time_data_in : in unsigned(63 downto 0);
		a_data_in : in signed(31 downto 0);
		a_en_in : in std_logic;
		s_s_en_in : in std_logic;
		c_s_en_in : in std_logic;
		av_s_en_in : in std_logic;
		a_u_en_in : in std_logic;
		s_u_en_in : in std_logic;
		c_u_en_in : in std_logic;
		av_u_en_in : in std_logic;
        full : out std_logic;
        pop : in std_logic;
        time_data_out : out unsigned(63 downto 0);
		a_data_out : out signed(31 downto 0);
		a_en_out : out std_logic;
		s_s_en_out : out std_logic;
		c_s_en_out : out std_logic;
		av_s_en_out : out std_logic;
		a_u_en_out : out std_logic;
		s_u_en_out : out std_logic;
		c_u_en_out : out std_logic;
		av_u_en_out : out std_logic;
        available : out std_logic
    );
end queue;

architecture behavioral of queue is

    signal is_full : std_logic;
    signal time_data_reg : unsigned64_array(1 downto 0);
    signal time_data : unsigned(63 downto 0);
	signal a_data_reg : signed32_array(1 downto 0);
	signal a_en_reg : bit_array(1 downto 0);
	signal a_data : signed(31 downto 0);
	signal a_en: std_logic;
	signal s_s_en_reg : bit_array(1 downto 0);
	signal s_s_en : std_logic;
	signal c_s_en_reg : bit_array(1 downto 0);
	signal c_s_en : std_logic;
	signal av_s_en_reg : bit_array(1 downto 0);
	signal av_s_en : std_logic;
	signal a_u_en_reg : bit_array(1 downto 0);
	signal a_u_en : std_logic;
	signal s_u_en_reg : bit_array(1 downto 0);
	signal s_u_en : std_logic;
	signal c_u_en_reg : bit_array(1 downto 0);
	signal c_u_en : std_logic;
	signal av_u_en_reg : bit_array(1 downto 0);
	signal av_u_en : std_logic;
    signal av : std_logic;
    signal size : integer;
    signal clk_reg : std_logic;
    signal push_done : std_logic;
    signal pop_done : std_logic;

begin

    process(rst, clk) begin
        if (rst = '1') then
            is_full <= '0';
            time_data_reg(time_data_reg'high downto 0) <= (others => (others => '0'));
            time_data <= (others => '0');
			a_data_reg(a_data_reg'high downto 0) <= (others => (others => '0'));
			a_en_reg(a_en_reg'high downto 0) <= (others => '0');
			a_data <= (others => '0');
			a_en <= '0';
			s_s_en_reg <= (others => '0');
			s_s_en <= '0';
			c_s_en_reg <= (others => '0');
			c_s_en <= '0';
			av_s_en_reg <= (others => '0');
			av_s_en <= '0';
			a_u_en_reg <= (others => '0');
			a_u_en <= '0';
			s_u_en_reg <= (others => '0');
			s_u_en <= '0';
			c_u_en_reg <= (others => '0');
			c_u_en <= '0';
			av_u_en_reg <= (others => '0');
			av_u_en <= '0';
            size <= 0;
            av <= '0';
            clk_reg <= '0';
            push_done <= '0';
            pop_done <= '0';
        elsif rising_edge(clk) then
            clk_reg <= not clk_reg;
            if clk_reg = '0' then
                if push = '1' and push_done = '0' and pop = '1' and pop_done = '0' and size > 0 and size < 2 then
                    -- perform push and pop
                    time_data_reg <= time_data_reg(time_data_reg'high - 1 downto 0) & time_data_in;
					a_data_reg <= a_data_reg(a_data_reg'high - 1 downto 0) & a_data_in;
					a_en_reg <= a_en_reg(a_en_reg'high - 1 downto 0) & a_en_in;
					s_s_en_reg <= s_s_en_reg(s_s_en_reg'high - 1 downto 0) & s_s_en_in;
					c_s_en_reg <= c_s_en_reg(c_s_en_reg'high - 1 downto 0) & c_s_en_in;
					av_s_en_reg <= av_s_en_reg(av_s_en_reg'high - 1 downto 0) & av_s_en_in;
					a_u_en_reg <= a_u_en_reg(a_u_en_reg'high - 1 downto 0) & a_u_en_in;
					s_u_en_reg <= s_u_en_reg(s_u_en_reg'high - 1 downto 0) & s_u_en_in;
					c_u_en_reg <= c_u_en_reg(c_u_en_reg'high - 1 downto 0) & c_u_en_in;
					av_u_en_reg <= av_u_en_reg(av_u_en_reg'high - 1 downto 0) & av_u_en_in;

                    time_data <= time_data_reg(size-1);
					a_data <= a_data_reg(size-1);
					a_en <= a_en_reg(size-1);
					s_s_en <= s_s_en_reg(size-1);
					c_s_en <= c_s_en_reg(size-1);
					av_s_en <= av_s_en_reg(size-1);
					a_u_en <= a_u_en_reg(size-1);
					s_u_en <= s_u_en_reg(size-1);
					c_u_en <= c_u_en_reg(size-1);
					av_u_en <= av_u_en_reg(size-1);
                    push_done <= '1';
                    pop_done <= '1';
                elsif push = '1' and push_done = '0' and size < 2 then
                    -- perform push
                    time_data_reg <= time_data_reg(time_data_reg'high - 1 downto 0) & time_data_in;
					a_data_reg <= a_data_reg(a_data_reg'high - 1 downto 0) & a_data_in;
					a_en_reg <= a_en_reg(a_en_reg'high - 1 downto 0) & a_en_in;
					s_s_en_reg <= s_s_en_reg(s_s_en_reg'high - 1 downto 0) & s_s_en_in;
					c_s_en_reg <= c_s_en_reg(c_s_en_reg'high - 1 downto 0) & c_s_en_in;
					av_s_en_reg <= av_s_en_reg(av_s_en_reg'high - 1 downto 0) & av_s_en_in;
					a_u_en_reg <= a_u_en_reg(a_u_en_reg'high - 1 downto 0) & a_u_en_in;
					s_u_en_reg <= s_u_en_reg(s_u_en_reg'high - 1 downto 0) & s_u_en_in;
					c_u_en_reg <= c_u_en_reg(c_u_en_reg'high - 1 downto 0) & c_u_en_in;
					av_u_en_reg <= av_u_en_reg(av_u_en_reg'high - 1 downto 0) & av_u_en_in;

                    size <= size + 1;
                    av <= '1';
                    is_full <= to_std_logic(size = 1);
                    push_done <= '1';
                elsif pop = '1' and pop_done = '0' and size > 0 then
                    --perform pop
                    time_data <= time_data_reg(size-1);
					a_data <= a_data_reg(size-1);
					a_en <= a_en_reg(size-1);
					s_s_en <= s_s_en_reg(size-1);
					c_s_en <= c_s_en_reg(size-1);
					av_s_en <= av_s_en_reg(size-1);
					a_u_en <= a_u_en_reg(size-1);
					s_u_en <= s_u_en_reg(size-1);
					c_u_en <= c_u_en_reg(size-1);
					av_u_en <= av_u_en_reg(size-1);

                    size <= size - 1;
                    is_full <= '0';
                    av <= to_std_logic(size > 1);
                    pop_done <= '1';
                end if;
            else
                if push = '0' then
                    push_done <= '0';
                end if;
                if pop = '0' then 
                    pop_done <= '0';
                end if;
            end if;
        end if;
    end process;

    full <= is_full;
    time_data_out <= time_data;
	a_data_out <= a_data;
	a_en_out <= a_en;
	s_s_en_out <= s_s_en;
	c_s_en_out <= c_s_en;
	av_s_en_out <= av_s_en;
	a_u_en_out <= a_u_en;
	s_u_en_out <= s_u_en;
	c_u_en_out <= c_u_en;
	av_u_en_out <= av_u_en;
    available <= av;

end behavioral;
